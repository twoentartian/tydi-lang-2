use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::Serialize;

use tydi_lang_parser::tydi_memory_representation::{self, Project};

use crate::json_representation_implementation::Implementation;
use crate::json_representation_logic_type::LogicType;
use crate::json_representation_streamlet::Streamlet;
use crate::name_conversion;

pub enum JsonRepresentation_item_type {
    Unknwon,
    LogicType(String),
    Streamlet(String),
    Implementation(String),
}

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

    pub fn append(&mut self, other: &mut JsonRepresentation) {
        self.streamlets.append(&mut other.streamlets);
        self.implementations.append(&mut other.implementations);
        self.logic_types.append(&mut other.logic_types);
    }
}

pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, target_var: Arc<RwLock<tydi_memory_representation::Variable>>) -> Result<(JsonRepresentation_item_type, JsonRepresentation), String> {
    let mut output_json_representation = JsonRepresentation::new();
    let mut output_json_representation_item_type = JsonRepresentation_item_type::Unknwon;
    
    let target_var_name = name_conversion::get_global_variable_name(target_var.clone());
    let var_value = target_var.read().unwrap().get_value();

    match &var_value {
        tydi_memory_representation::TypedValue::LogicTypeValue(_) => {
            let (_, mut type_dependencies) = LogicType::translate_from_tydi_project(tydi_project.clone(), target_var.clone())?;
            output_json_representation.logic_types.append(&mut type_dependencies);
            output_json_representation_item_type = JsonRepresentation_item_type::LogicType(target_var_name);    //dirty way, will it cause bug in the future?
        },
        tydi_memory_representation::TypedValue::Streamlet(_) => {
            return Err(format!("{} (streamlet) is an invalid start point", &target_var_name));
        },
        tydi_memory_representation::TypedValue::Implementation(implementation) => {
            let (target_impl, mut type_dependencies) = Implementation::translate_from_tydi_project(tydi_project.clone(), target_var.clone())?;
            output_json_representation.append(&mut type_dependencies);
            output_json_representation_item_type = JsonRepresentation_item_type::Implementation(target_impl.read().unwrap().name.clone());
        },
        tydi_memory_representation::TypedValue::Array(_) => {
            return Err(format!("{} (array) is an invalid start point", &target_var_name));
        },
        tydi_memory_representation::TypedValue::RefToVar(var) => {
            let (var_type, mut var_dependencies) = translate_from_tydi_project(tydi_project.clone(), var.clone())?;
            output_json_representation.append(&mut var_dependencies);
            match var_type {
                JsonRepresentation_item_type::Unknwon => unreachable!("unknown variable type"),
                JsonRepresentation_item_type::LogicType(logic_type_name) => {
                    output_json_representation.logic_types.insert(target_var_name, Arc::new(RwLock::new(LogicType::Ref(logic_type_name))));
                },
                JsonRepresentation_item_type::Streamlet(streamlet_name) => {
                    todo!()
                },
                JsonRepresentation_item_type::Implementation(impl_name) => {
                    todo!()
                },
            }
        },
        _ => {
            return Err(format!("unsupported type, {} is an invalid start point", &target_var_name));
        }
    }


    return Ok((output_json_representation_item_type, output_json_representation));
}