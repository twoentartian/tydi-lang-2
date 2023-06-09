use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};

use crate::generate_name::generate_init_value;
use crate::tydi_memory_representation::{TemplateArg, CodeLocation, Scope, ScopeType, GetScope, Attribute, TraitCodeLocationAccess, Variable, TypeIndication};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub, generate_name};


#[derive(Clone, Debug, Serialize)]
pub struct FunctionCall {
    name: String,

    function_id: String,

    #[serde(with = "crate::serde_serialization::arc_rwlock_in_btree_map_value")]
    function_arg_exps: BTreeMap<usize, Arc<RwLock<Variable>>>,

    location_define: CodeLocation,
}

impl GetName for FunctionCall {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl TraitCodeLocationAccess for FunctionCall {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl FunctionCall {
    pub fn new(name: String) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name.clone(),
            function_id: generate_init_value(),
            function_arg_exps: BTreeMap::new(),
            location_define: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn add_function_arg_exp(&mut self, arg_exp: Arc<RwLock<Variable>>) {
        let current_index = self.function_arg_exps.len();
        self.function_arg_exps.insert(current_index, arg_exp);
    }

    generate_access_pub!(function_id, String, get_function_id, set_function_id);
    generate_access_pub!(function_arg_exps, BTreeMap<usize, Arc<RwLock<Variable>>>, get_function_arg_exps, set_function_arg_exps);
}

