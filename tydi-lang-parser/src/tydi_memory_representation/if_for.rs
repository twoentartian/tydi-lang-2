use std::sync::{Arc, RwLock};

use std::collections::BTreeMap;

use serde::{Serialize};

use crate::tydi_memory_representation::{Scope, GetScope, CodeLocation, TraitCodeLocationAccess, Variable};
use crate::trait_common::{GetName};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub};

#[derive(Clone, Debug, Serialize)]
pub struct If {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    if_exp: Arc<RwLock<Variable>>,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    if_scope: Arc<RwLock<Scope>>,

    elif_blocks: BTreeMap<usize, Elif>,

    else_block: Option<Else>,
    
    location_define: CodeLocation,
}

impl GetName for If {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl TraitCodeLocationAccess for If {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl GetScope for If {
    generate_get!(if_scope, Arc<RwLock<Scope>>, get_scope);
}

impl If {
    pub fn new(name: String, parent_scope: Arc<RwLock<Scope>>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name.clone(),
            if_exp: Variable::new_place_holder(),
            if_scope: Scope::new(format!("if_{}", &name), super::ScopeType::IfForScope, parent_scope),
            elif_blocks: BTreeMap::new(),
            else_block: None,
            location_define: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    generate_set_pub!(name, String, set_name);
    generate_access_pub!(if_exp, Arc<RwLock<Variable>>, get_if_exp, set_if_exp);
}


#[derive(Clone, Debug, Serialize)]
pub struct Elif {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    elif_exp: Arc<RwLock<Variable>>,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    elif_scope: Arc<RwLock<Scope>>,

    location_define: CodeLocation,
}

impl TraitCodeLocationAccess for Elif {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl GetScope for Elif {
    generate_get!(elif_scope, Arc<RwLock<Scope>>, get_scope);
}

impl Elif {
    pub fn new(name: String, parent_scope: Arc<RwLock<Scope>>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name.clone(),
            elif_exp: Variable::new_place_holder(),
            elif_scope: Scope::new(format!("elif_{}", &name), super::ScopeType::IfForScope, parent_scope),
            location_define: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    generate_set_pub!(name, String, set_name);
    generate_access_pub!(elif_exp, Arc<RwLock<Variable>>, get_elif_exp, set_elif_exp);
}

#[derive(Clone, Debug, Serialize)]
pub struct Else {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    else_scope: Arc<RwLock<Scope>>,

    location_define: CodeLocation,
}

impl TraitCodeLocationAccess for Else {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl GetScope for Else {
    generate_get!(else_scope, Arc<RwLock<Scope>>, get_scope);
}

impl Else {
    pub fn new(name: String, parent_scope: Arc<RwLock<Scope>>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name.clone(),
            else_scope: Scope::new(format!("else_{}", &name), super::ScopeType::IfForScope, parent_scope),
            location_define: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    generate_set_pub!(name, String, set_name);
}



#[derive(Clone, Debug, Serialize)]
pub struct For {

}