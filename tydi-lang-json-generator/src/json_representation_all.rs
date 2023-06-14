use std::clone;
use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};
use serde::ser::SerializeStruct;


use crate::json_representation_logic_type::{LogicType};


#[derive(Clone, Debug, Serialize)]
pub struct JsonRepresentation {
    #[serde(with = "crate::serde_serialization::arc_rwlock_in_btree_map_value")]
    pub logic_types: BTreeMap<String, Arc<RwLock<LogicType>>>,
}

impl JsonRepresentation {
    pub fn new() -> Self {
        return Self {
            logic_types: BTreeMap::new(),
        };
    }
}
