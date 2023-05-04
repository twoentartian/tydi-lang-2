use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::{Serialize, Deserialize};

use crate::error::TydiLangError;
use crate::{generate_get, generate_name};
use crate::trait_common::{GetName};
use crate::tydi_memory_representation::{Variable};

use super::TraitCodeLocationAccess;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ScopeRelationType {
    FileScopeRela,

    GroupScopeRela,
    UnionScopeRela,
    StreamletScopeRela,
    ImplementationScopeRela,
    IfForScopeRela,

    ParentScopeRela, // a placeholder, should never be used
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
    scope_relationships: HashMap<String, ScopeRelationship>,

    #[serde(with = "crate::serde_serialization::arc_rwlock_in_map_value")]
    variables: HashMap<String, Arc<RwLock<Variable>>>
}

impl Scope {
    pub fn new(name: String, scope_type: ScopeType, parent_scope: Arc<RwLock<Self>>) -> Arc<RwLock<Self>> {
        let output = Arc::new(RwLock::new(Scope {
            name: name,
            scope_type: scope_type,
            self_ref: None,
            scope_relationships: HashMap::new(),
            variables: HashMap::new(),
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
            scope_relationships: HashMap::new(),
            variables: HashMap::new(),
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
            scope_relationships: HashMap::new(),
            variables: HashMap::new(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_var(&mut self, var: Arc<RwLock<Variable>>) -> Result<(), TydiLangError> {
        let var_name = var.read().unwrap().get_name();
        if self.variables.contains_key(&var_name) {
            let error = TydiLangError::new(format!("{var_name} redefined in the same scope"), var.read().unwrap().get_code_location());
            return Err(error);
        }
        self.variables.insert(var_name, var.clone());
        return Ok(());
    }

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
}

impl GetName for Scope {
    generate_get!(name, String, get_name);
}

pub trait GetScope {
    fn get_scope(&self) -> Arc<RwLock<Scope>>;
}

#[cfg(test)]
mod test_scope {
use super::*;

    #[test]
    fn create_serialize_scopes() {
        let root_scope = Scope::new_top_scope(format!("root"));
        let child_scope_0 = Scope::new(
            format!("child_scope_0"),
            ScopeType::GroupScope,
            root_scope.clone(),
        );
        let child_scope_1 = Scope::new(
            format!("child_scope_1"),
            ScopeType::IfForScope,
            child_scope_0.clone(),
        );
    
        {
            let root_scope_read = root_scope.read().unwrap();
            let json_output = serde_json::to_string(&*root_scope_read).ok().unwrap();
            println!("{json_output}");
            assert_eq!(json_output, r#"{"name":"root","scope_type":"RootScope","scope_relationships":{}}"#)
        }
    
        {
            let child_scope_0 = child_scope_0.read().unwrap();
            let json_output = serde_json::to_string(&*child_scope_0).ok().unwrap();
            println!("{json_output}");
            assert_eq!(json_output, r#"{"name":"child_scope_0","scope_type":"GroupScope","scope_relationships":{"root":{"name":"root","target_scope":"root","relationship":"GroupScopeRela"}}}"#)
        }
    
        {
            let child_scope_1 = child_scope_1.read().unwrap();
            let json_output = serde_json::to_string(&*child_scope_1).ok().unwrap();
            println!("{json_output}");
            assert_eq!(json_output, r#"{"name":"child_scope_1","scope_type":"IfForScope","scope_relationships":{"child_scope_0":{"name":"child_scope_0","target_scope":"child_scope_0","relationship":"IfForScopeRela"}}}"#)
        }
    
    
    }
}



