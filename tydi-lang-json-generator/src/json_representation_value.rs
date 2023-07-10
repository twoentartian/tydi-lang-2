use std::sync::{Arc, RwLock};

use serde::{Serialize};
use tydi_lang_parser::tydi_memory_representation::{TypedValue, Project};

use crate::{json_representation_logic_type::LogicType, json_representation_all::JsonRepresentation, util::generate_random_str};



#[derive(Clone, Debug, Serialize)]
pub enum Value {
    Int(i128),
    Float(f64),
    Bool(bool),
    String(String),
    ClockDomain(String),

    LogicType(LogicType),

    Array(Vec<Value>),
}

impl Value {
    pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, value: &TypedValue) -> Result<(Value, JsonRepresentation), String> {
        let mut output_dependency = JsonRepresentation::new();

        match &value {
            TypedValue::IntValue(v) => return Ok((Value::Int(*v), output_dependency)),
            TypedValue::StringValue(v) => return Ok((Value::String(v.clone()), output_dependency)),
            TypedValue::BoolValue(v) => return Ok((Value::Bool(*v), output_dependency)),
            TypedValue::FloatValue(v) => return Ok((Value::Float(*v), output_dependency)),
            TypedValue::ClockDomainValue(v) => return Ok((Value::ClockDomain(v.clone()), output_dependency)),
            TypedValue::LogicTypeValue(_) | TypedValue::RefToVar(_) => {
                let (output_value, mut dependencies) = LogicType::translate_from_tydi_project_type_value(tydi_project, value, generate_random_str(8))?;
                output_dependency.logic_types.append(&mut dependencies);
                return Ok((Value::LogicType(output_value), output_dependency));
            }
            _ => unreachable!(),
        }
    }
}