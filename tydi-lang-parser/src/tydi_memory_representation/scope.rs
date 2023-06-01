use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, RwLock};

use serde::{Serialize, Deserialize};

use crate::error::TydiLangError;
use crate::{generate_get, generate_name, generate_get_ref_pub, generate_get_pub};
use crate::trait_common::{GetName};
use crate::tydi_memory_representation::{Variable, CodeLocation};

use super::TraitCodeLocationAccess;

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
        return output;
    }

    pub fn resolve_id_in_current_scope() -> HashSet<ScopeRelationType> {
        let mut output = HashSet::new();
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

#[derive(Clone, Debug, Serialize)]
pub struct Scope {
    name: String,
    pub scope_type: ScopeType,

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

impl Scope {

    //constructors
    pub fn new(name: String, scope_type: ScopeType, parent_scope: Arc<RwLock<Self>>) -> Arc<RwLock<Self>> {
        let output = Arc::new(RwLock::new(Scope {
            name: name,
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

    generate_get_pub!(variables, BTreeMap<String, Arc<RwLock<Variable>>>, get_variables);
    generate_get_ref_pub!(variables, BTreeMap<String, Arc<RwLock<Variable>>>, get_variables_ref);
    generate_get_ref_pub!(scope_relationships, BTreeMap<String, ScopeRelationship>, get_scope_relationships);

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
    pub fn resolve_identifier(name: &String, scope: Arc<RwLock<Scope>>, scope_relation_types/*allowed edges*/: HashSet<ScopeRelationType>) -> Result<(Arc<RwLock<Variable>>, Arc<RwLock<Scope>>), TydiLangError> {
        let scope_read = scope.read().unwrap();
        
        //does current scope has this var?
        let result = Scope::resolve_identifier_in_current_scope(&name, scope.clone());
        if result.is_some() {
            return Ok((result.unwrap(), scope.clone()));
        }

        //how about other scopes?
        for (_, item) in scope_read.get_scope_relationships() {
            let (other_scope, relationship_type) = (item.target_scope.clone(), &item.relationship);
            if scope_relation_types.contains(relationship_type) {
                let result = Scope::resolve_identifier(name, other_scope, scope_relation_types)?;
                return Ok(result);
            }
        }

        return Err(TydiLangError::new(format!("identifier {} not found in scope {}", &name, scope.read().unwrap().get_name()), CodeLocation::new_unknown()));
    }

    fn resolve_identifier_in_current_scope(name: &String, scope: Arc<RwLock<Scope>>) -> Option<Arc<RwLock<Variable>>> {
        let scope_read = scope.read().unwrap();
        let vars_in_scope = scope_read.get_variables_ref();
        match vars_in_scope.get(name) {
            Some(var) => return Some(var.clone()),
            None => return None,
        }
    }
}

pub trait GetScope {
    fn get_scope(&self) -> Arc<RwLock<Scope>>;
}

