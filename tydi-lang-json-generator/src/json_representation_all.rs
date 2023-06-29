use std::clone;
use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};
use serde::ser::SerializeStruct;

use tydi_lang_parser::tydi_memory_representation::{self, Project};

use crate::json_representation_implementation::Implementation;
use crate::json_representation_logic_type::{LogicType};
use crate::json_representation_streamlet::Streamlet;


#[derive(Clone, Debug, Serialize)]
pub struct JsonRepresentation {
    #[serde(with = "crate::serde_serialization::arc_rwlock_in_btree_map_value")]
    pub logic_types: BTreeMap<String, Arc<RwLock<LogicType>>>,
    #[serde(with = "crate::serde_serialization::arc_rwlock_in_btree_map_value")]
    pub streamlets: BTreeMap<String, Arc<RwLock<Streamlet>>>,
    #[serde(with = "crate::serde_serialization::arc_rwlock_in_btree_map_value")]
    pub implementations: BTreeMap<String, Arc<RwLock<Implementation>>>,
}

impl JsonRepresentation {
    pub fn new() -> Self {
        return Self {
            logic_types: BTreeMap::new(),
            streamlets: BTreeMap::new(),
            implementations: BTreeMap::new(),
        };
    }
}

pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, target_var: Arc<RwLock<tydi_memory_representation::Variable>>) -> Result<JsonRepresentation, String> {
    todo!()
}