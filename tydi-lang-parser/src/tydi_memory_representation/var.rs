use core::str;
use std::sync::{Arc, RwLock};

use serde::{Serialize, Serializer, Deserialize};

use crate::tydi_memory_representation::{TypedValue, TypeIndication};
use crate::name_trait::GetName;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EvaluationStatus {
    NotEvaluated,
    EvaluationCount(u32),
    Evaluated,
    Predefined,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,

    pub exp: Option<String>,

    pub evaluated: EvaluationStatus,

    pub value: Vec<TypedValue>,     //the variable can be an array

    pub is_array: bool,

    pub type_indication: TypeIndication,
}

impl GetName for Variable {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl Serialize for Variable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        use serde::ser::SerializeStruct;
        if self.evaluated == EvaluationStatus::Evaluated || self.evaluated == EvaluationStatus::Predefined {
            if self.is_array {
                
            }
            else {

            }
        }
        else {
            let mut state = serializer.serialize_struct("Variable", 4)?;
            state.serialize_field("name", &self.name)?;
            state.serialize_field("exp", &self.exp)?;
            state.serialize_field("is_array", &self.is_array)?;
            state.serialize_field("type_indication", &self.type_indication)?;
            state.end()
        }
    }
}

impl Variable {
    pub fn new(name: String, exp: Option<String>) -> Arc<RwLock<Variable>> {
        let mut output = Variable {
            name: name,
            exp: exp,
            evaluated: EvaluationStatus::NotEvaluated,
            value: vec![],
            is_array: false,
            type_indication: TypeIndication::Any,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_predefined(name: String, value: TypedValue) -> Arc<RwLock<Variable>> {
        let mut output = Variable {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::Predefined,
            value: vec![value.clone()],
            is_array: false,
            type_indication: TypeIndication::infer_from_typed_value(value),
        };
        return Arc::new(RwLock::new(output));
    }
}