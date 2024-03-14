use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, RwLock};

use serde::{Serialize, Deserialize};

use crate::deep_clone::{DeepClone, DeepClone_ArcLock};
use crate::error::TydiLangError;
use crate::evaluation::{Evaluator, template_expansion};
use crate::{generate_get, generate_name, generate_get_ref_pub, generate_get_pub, generate_set_pub, generate_access_pub};
use crate::trait_common::GetName;
use crate::tydi_memory_representation::{Variable, CodeLocation, TraitCodeLocationAccess, TypedValue, LogicType};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum ScopeRelationType {
    FileScopeRela,

    GroupScopeRela,
    UnionScopeRela,
    StreamletScopeRela,
    ImplementationScopeRela,
    IfForScopeRela,

    ImplToStreamletRela,

    ParentScopeRela, // a placeholder, should never be used
}

impl ScopeRelationType {
    pub fn resolve_id_default() -> HashSet<ScopeRelationType> {
        let mut output = HashSet::new();
        output.insert(ScopeRelationType::GroupScopeRela);
        output.insert(ScopeRelationType::UnionScopeRela);
        output.insert(ScopeRelationType::StreamletScopeRela);
        output.insert(ScopeRelationType::ImplementationScopeRela);
        output.insert(ScopeRelationType::IfForScopeRela);
        return output;
    }

    pub fn resolve_id_in_current_scope() -> HashSet<ScopeRelationType> {
        let output = HashSet::new();
        return output;
    }

    pub fn resolve_id_in_parent_streamlet() -> HashSet<ScopeRelationType> {
        let mut output = HashSet::new();
        output.insert(ScopeRelationType::ImplToStreamletRela);
        return output;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ScopeType {
    RootScope,

    FileScope,

    GroupScope,
    UnionScope,
    StreamletScope,
    ImplementationScope,
    IfForScope,

    ParentScope, // a placeholder, should never be used
    UnknownScope,
}

#[derive(Clone, Debug, Serialize)]
pub struct ScopeRelationship {
    name: String,
    #[serde(with = "crate::serde_serialization::use_name_for_arc_rwlock")]
    target_scope: Arc<RwLock<Scope>>,
    relationship: ScopeRelationType,
}

impl DeepClone for ScopeRelationship {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl ScopeRelationship {
    pub fn new(target_scope: Arc<RwLock<Scope>>, relationship: ScopeRelationType) -> Self {
        let output = Self {
            name: target_scope.read().unwrap().get_name(),
            target_scope: target_scope.clone(),
            relationship: relationship,
        };
        return output;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Scope {
    name: String,

    id_in_code: Option<String>,

    scope_type: ScopeType,

    #[serde(skip)]
    self_ref: Option<Arc<RwLock<Scope>>>,

    //// HashMap < parent_scope_name , scope_relationship >
    scope_relationships: BTreeMap<String, ScopeRelationship>,

    #[serde(with = "crate::serde_serialization::arc_rwlock_in_btree_map_value")]
    variables: BTreeMap<String, Arc<RwLock<Variable>>>
}

impl GetName for Scope {
    generate_get!(name, String, get_name);
}

impl DeepClone_ArcLock for Scope {
    fn deep_clone_arclock(&self) -> Arc<RwLock<Self>> {
        let output = Self {
            name: self.name.deep_clone(),
            id_in_code: self.id_in_code.deep_clone(),
            scope_type: self.scope_type.clone(),
            self_ref: None,
            scope_relationships: self.scope_relationships.deep_clone(), //Notice: this is a dirty implementation, user needs to maintain the scope relationships by themselves
            variables: self.variables.deep_clone(),
        };

        let output = Arc::new(RwLock::new(output));
        {
            let mut output_write = output.write().unwrap();
            output_write.self_ref = Some(output.clone());
        }

        //maintain the scope relationships
        {
            let variables = output.read().unwrap().get_variables();
            for (_, var) in variables {
                let var_value = var.read().unwrap().get_value();
                let remove_old_scope_add_new_scope = |scope: Arc<RwLock<Scope>>, scope_rela_type: ScopeRelationType, target_scope: Arc<RwLock<Scope>>| {
                    let mut current_relationships = scope.read().unwrap().get_scope_relationships().clone();
                    {   //remove old relationship
                        let mut rela_to_remove = vec![];
                        for (name, rela_type) in &current_relationships {
                            if rela_type.relationship == scope_rela_type {
                                rela_to_remove.push(name.clone());
                            }
                        }
                        for i in rela_to_remove {
                            current_relationships.remove(&i);
                        }
                    }
                    {   //add new relationship
                        let scope_relationship = ScopeRelationship::new(target_scope.clone(), scope_rela_type);
                        current_relationships.insert(self.get_name(), scope_relationship);
                    }
                    scope.write().unwrap().set_scope_relationships(current_relationships);
                };
                match var_value {
                    TypedValue::LogicTypeValue(logic_type) => {
                        let logic_type_value = logic_type.write().unwrap();
                        match &*logic_type_value {
                            LogicType::LogicGroupType(group) => {
                                let group_scope = group.read().unwrap().get_scope();
                                remove_old_scope_add_new_scope(group_scope.clone(), ScopeRelationType::GroupScopeRela, output.clone());
                            },
                            LogicType::LogicUnionType(union) => {
                                let union_scope = union.read().unwrap().get_scope();
                                remove_old_scope_add_new_scope(union_scope.clone(), ScopeRelationType::GroupScopeRela, output.clone());
                            },
                            _ => () //other logic types don't have scope
                        }
                    },
                    TypedValue::Streamlet(target_streamlet) => {
                        let streamlet_scope = target_streamlet.read().unwrap().get_scope();
                        remove_old_scope_add_new_scope(streamlet_scope.clone(), ScopeRelationType::StreamletScopeRela, output.clone());
                    },
                    TypedValue::Implementation(target_implementation) => {
                        let implementation_scope = target_implementation.read().unwrap().get_scope();
                        remove_old_scope_add_new_scope(implementation_scope.clone(), ScopeRelationType::ImplementationScopeRela, output.clone());
                    },
                    TypedValue::If(target_if) => {
                        let if_scope = target_if.read().unwrap().get_scope();
                        remove_old_scope_add_new_scope(if_scope.clone(), ScopeRelationType::IfForScopeRela, output.clone());
                    },
                    TypedValue::For(target_for) => {
                        let for_scope = target_for.read().unwrap().get_scope();
                        remove_old_scope_add_new_scope(for_scope.clone(), ScopeRelationType::IfForScopeRela, output.clone());
                    },
                    _ => (),    //other typed values don't have scope
                }
            }
        }

        //set variable parent scope
        {
            let variables = output.read().unwrap().get_variables();
            for (_, var) in variables {
                var.write().unwrap().set_parent_scope(Some(output.clone()));
            }
        }

        return output;
    }
}

impl Scope {

    //constructors
    pub fn new(name: String, scope_type: ScopeType, parent_scope: Arc<RwLock<Self>>) -> Arc<RwLock<Self>> {
        let output = Arc::new(RwLock::new(Scope {
            name: name,
            id_in_code: None,
            scope_type: scope_type,
            self_ref: None,
            scope_relationships: BTreeMap::new(),
            variables: BTreeMap::new(),
        }));

        {
            let mut output_write = output.write().unwrap();
            output_write.self_ref = Some(output.clone());
            let parent_scaope_rela = ScopeRelationship {
                name: parent_scope.read().unwrap().get_name(),
                target_scope: parent_scope.clone(),
                relationship: output_write.generate_scope_relationship(),
            };
            output_write.scope_relationships.insert(parent_scaope_rela.name.clone(), parent_scaope_rela);
        }

        return output;
    }

    pub fn new_top_scope(name: String) -> Arc<RwLock<Self>> {
        let output = Arc::new(RwLock::new(Scope {
            name: name,
            id_in_code: None,
            scope_type: ScopeType::RootScope,
            self_ref: None,
            scope_relationships: BTreeMap::new(),
            variables: BTreeMap::new(),
        }));

        {
            let mut output_write = output.write().unwrap();
            output_write.self_ref = Some(output.clone());
        }

        return output.clone();
    }

    pub fn new_place_holder() -> Arc<RwLock<Self>> {
        let output = Self {
            name: generate_name::generate_init_value(),
            id_in_code: None,
            scope_type: ScopeType::UnknownScope,
            self_ref: None,
            scope_relationships: BTreeMap::new(),
            variables: BTreeMap::new(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_var(&mut self, var: Arc<RwLock<Variable>>) -> Result<(), TydiLangError> {
        let var_name = var.read().unwrap().get_name();
        let previous_define = self.variables.get(&var_name);
        match previous_define {
            None => (),
            Some(previous_define) => {
                let previous_define_loc = previous_define.read().unwrap().get_code_location();
                let error = TydiLangError::new_multiple_locations(format!("{var_name} redefined in the same scope {}", self.get_name()), vec![previous_define_loc, var.read().unwrap().get_code_location()]);
                return Err(error);
            },
        }
        self.variables.insert(var_name, var.clone());
        //set parent scope of variable
        {
            assert!(self.self_ref.is_some());
            var.write().unwrap().set_parent_scope(Some(self.self_ref.as_ref().unwrap().clone()));
        }
        return Ok(());
    }

    pub fn add_scope_relationship(&mut self, target_scope: Arc<RwLock<Scope>>, relationship_type: ScopeRelationType) -> Result<(), TydiLangError> {
        let name = target_scope.read().unwrap().get_name();
        let rela_target = ScopeRelationship {
            name: name.clone(),
            target_scope: target_scope.clone(),
            relationship: relationship_type,
        };
        self.scope_relationships.insert(name.clone(), rela_target);
        return Ok(());
    }

    // pub fn get_variables(&self) -> &HashMap<String, Arc<RwLock<Variable>>> {
    //     return &self.variables;
    // }

    // pub fn get_scope_relationships(&self) -> &HashMap<String, ScopeRelationship> {
    //     return &self.scope_relationships;
    // }

    generate_access_pub!(variables, BTreeMap<String, Arc<RwLock<Variable>>>, get_variables, set_variables);
    generate_get_pub!(scope_type, ScopeType, get_scope_type);
    generate_get_ref_pub!(variables, BTreeMap<String, Arc<RwLock<Variable>>>, get_variables_ref);
    generate_get_ref_pub!(scope_relationships, BTreeMap<String, ScopeRelationship>, get_scope_relationships);
    generate_set_pub!(scope_relationships, BTreeMap<String, ScopeRelationship>, set_scope_relationships);

    fn generate_scope_relationship(&self) -> ScopeRelationType {
        match self.scope_type {
            ScopeType::RootScope => todo!(),
            ScopeType::FileScope => ScopeRelationType::FileScopeRela,
            ScopeType::GroupScope => ScopeRelationType::GroupScopeRela,
            ScopeType::UnionScope => ScopeRelationType::UnionScopeRela,
            ScopeType::StreamletScope => ScopeRelationType::StreamletScopeRela,
            ScopeType::ImplementationScope => ScopeRelationType::ImplementationScopeRela,
            ScopeType::IfForScope => ScopeRelationType::IfForScopeRela,
            ScopeType::ParentScope => ScopeRelationType::ParentScopeRela,
            ScopeType::UnknownScope => todo!(),
        }
    }

    //resolve identifier
    pub fn resolve_identifier(name: &String, template_exps: &Option<BTreeMap<usize, TypedValue>>, location: &CodeLocation, scope: Arc<RwLock<Scope>>, template_expansion_scope: Arc<RwLock<Scope>>, scope_relation_types/*allowed edges*/: HashSet<ScopeRelationType>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(Arc<RwLock<Variable>>, Arc<RwLock<Scope>>), TydiLangError> {        
        //does current scope has this var?
        let result = Scope::resolve_identifier_in_current_scope(&name, &template_exps, location, scope.clone(), template_expansion_scope.clone(), evaluator.clone())?;
        if result.is_some() {
            return Ok((result.unwrap(), scope.clone()));
        }

        //how about other scopes?
        let other_scope_relationships = scope.read().unwrap().get_scope_relationships().clone();
        for (_, item) in other_scope_relationships {
            let (other_scope, relationship_type) = (item.target_scope, item.relationship);
            if scope_relation_types.contains(&relationship_type) {
                let result = Scope::resolve_identifier(name, template_exps, location, other_scope, template_expansion_scope.clone(), scope_relation_types, evaluator.clone())?;
                return Ok(result);
            }
        }

        return Err(TydiLangError::new(format!("identifier {} not found in scope {}", &name, scope.read().unwrap().get_name()), location.clone()));
    }

    fn resolve_identifier_in_current_scope(name: &String, template_exps: &Option<BTreeMap<usize, TypedValue>>, location: &CodeLocation, scope: Arc<RwLock<Scope>>, template_expansion_scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<Option<Arc<RwLock<Variable>>>, TydiLangError> {
        let identifier_var = match scope.read().unwrap().get_variables_ref().get(name) {
            Some(var) => var.clone(),
            None => return Ok(None),
        };

        //this is a template instance
        let output_var = template_expansion::try_template_expansion(identifier_var.clone(), template_exps, template_expansion_scope.clone(), evaluator.clone())?;
        return Ok(Some(output_var));
    }
}

pub trait GetScope {
    fn get_scope(&self) -> Arc<RwLock<Scope>>;
}

pub trait GlobalIdentifier {
    fn set_parent_scope(&mut self, parent_scope: Option<Arc<RwLock<Scope>>>);

    fn get_parent_scope(&self) -> Option<Arc<RwLock<Scope>>>;

    fn set_id_in_scope(&mut self, id: Option<String>);

    fn get_id_in_scope(&self) -> Option<String>;
}