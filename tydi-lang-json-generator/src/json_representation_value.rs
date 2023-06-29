use serde::{Serialize};

use crate::json_representation_logic_type::LogicType;



#[derive(Clone, Debug, Serialize)]
pub enum Value {
    Int(i128),
    Float(f64),
    Bool(bool),
    String(String),

    LogicType(LogicType),

    Array(Vec<Value>),
}