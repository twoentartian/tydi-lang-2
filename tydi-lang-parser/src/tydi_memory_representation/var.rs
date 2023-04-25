use core::str;
use std::sync::{Arc, RwLock};

use serde::{Serialize, Serializer, Deserialize};

use crate::tydi_memory_representation::{TypedValue, TypeIndication, CodeLocation, TraitCodeLocationAccess};
use crate::trait_common::GetName;
use crate::{generate_get_pub, generate_access_pub, generate_set_pub, generate_name};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EvaluationStatus {
    NotEvaluated,
    EvaluationCount(u32),
    Evaluated,
    Predefined,
}

#[derive(Clone, Debug)]
pub struct Variable {
    name: String,
    exp: Option<String>,
    evaluated: EvaluationStatus,
    value: Vec<TypedValue>,     //the variable can be an array
    is_array: bool,
    type_indication: TypeIndication,
    is_property_of_scope: bool,
    declare_location: CodeLocation,
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
        use serde::ser::{SerializeStruct, SerializeSeq};
        if self.evaluated == EvaluationStatus::Evaluated || self.evaluated == EvaluationStatus::Predefined {
            if self.is_array {
                let mut seq = serializer.serialize_seq(Some(self.value.len()))?;
                for value in &self.value {
                    seq.serialize_element(&value)?;
                }
                seq.end()
            }
            else {
                let value = &self.value[0];
                value.serialize(serializer)
            }
        }
        else {
            let mut state = serializer.serialize_struct("Variable", 6)?;
            state.serialize_field("name", &self.name)?;
            state.serialize_field("exp", &self.exp)?;
            state.serialize_field("value", &self.value)?;
            state.serialize_field("evaluated", &self.evaluated)?;
            state.serialize_field("is_array", &self.is_array)?;
            state.serialize_field("type_indication", &self.type_indication)?;
            state.serialize_field("is_property_of_scope", &self.is_property_of_scope)?;
            state.serialize_field("declare_location", &self.declare_location)?;
            state.end()
        }
    }
}

impl TraitCodeLocationAccess for Variable {
    fn set_code_location(& mut self, loc: CodeLocation) {
        self.declare_location = loc;
    }

    fn get_code_location(&self) -> CodeLocation {
        return self.declare_location.clone();
    }
}

impl Variable {
    pub fn new(name: String, exp: Option<String>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            exp: exp,
            evaluated: EvaluationStatus::NotEvaluated,
            value: vec![],
            is_array: false,
            type_indication: TypeIndication::Any,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_place_holder() -> Arc<RwLock<Self>> {
        let output = Self {
            name: generate_name::generate_init_value(),
            exp: Some(generate_name::generate_init_value()),
            evaluated: EvaluationStatus::NotEvaluated,
            value: vec![],
            is_array: false,
            type_indication: TypeIndication::Unknown,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_logic_type(name: String, value: TypedValue) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::NotEvaluated,
            value: vec![value],
            is_array: false,
            type_indication: TypeIndication::AnyLogicType,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_predefined(name: String, value: TypedValue) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::Predefined,
            value: vec![value.clone()],
            is_array: false,
            type_indication: TypeIndication::infer_from_typed_value(&value),
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_predefined_empty_array(name: String, type_indication: TypeIndication) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::Predefined,
            value: vec![],
            is_array: true,
            type_indication: type_indication,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn add_predefined_element(&mut self, value: TypedValue) -> Result<&mut Self, String> {
        if !self.is_array {
            return Err(format!("{} is not an array type", &self.name));
        }
        if !self.type_indication.is_compatible_with_typed_value(&value) {
            return Err(format!("type mismatch, array type: {:?}, element type： {:?}", self.type_indication, value));
        }

        //change the type indicator?
        if self.value.len() == 0 {
            self.type_indication = TypeIndication::infer_from_typed_value(&value);
        }

        self.value.push(value);
        return Ok(self);
    }

    generate_set_pub!(name, String, set_name);
    generate_access_pub!(exp, Option<String>, get_exp, set_exp);
    generate_access_pub!(value, Vec<TypedValue>, get_value, set_value);
    generate_access_pub!(type_indication, TypeIndication, get_type_indication, set_type_indication);
    generate_access_pub!(is_array, bool, get_is_array, set_is_array);
    generate_access_pub!(is_property_of_scope, bool, get_is_property_of_scope, set_is_property_of_scope);
    generate_get_pub!(evaluated, EvaluationStatus, get_evaluated);
}


#[cfg(test)]
mod test_var {
    use super::*;

    #[test]
    fn serialize_variable() {
        let value = Variable::new_predefined(format!("value"), TypedValue::BoolValue(true));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#"true"#);

        let value = Variable::new_predefined(format!("value"), TypedValue::IntValue(100));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#"100"#);

        let value = Variable::new_predefined(format!("value"), TypedValue::StringValue(format!("123")));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#""123""#);

        let value = Variable::new_predefined(format!("value"), TypedValue::FloatValue(99.99));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#"99.99"#);

        let value = Variable::new_predefined(format!("value"), TypedValue::ClockDomainValue(format!("test_clock_domain")));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#""CLOCK_test_clock_domain""#);
    }

    #[test]
    fn serialize_variable_array() {
        let value = Variable::new_predefined_empty_array(format!("value"), TypeIndication::Int);
        {
            let mut value_write = value.write().unwrap();
            value_write.add_predefined_element(TypedValue::IntValue(10)).unwrap();
            value_write.add_predefined_element(TypedValue::IntValue(15)).unwrap();
            value_write.add_predefined_element(TypedValue::IntValue(20)).unwrap();
        }
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#"[10,15,20]"#);

        //type mismatch
        let value = Variable::new_predefined_empty_array(format!("value"), TypeIndication::Int);
        {
            let mut value_write = value.write().unwrap();
            let result = value_write.add_predefined_element(TypedValue::FloatValue(10.0));
            assert!(result.is_err());
            println!("{}", result.err().unwrap());
        }
        let value = Variable::new_predefined_empty_array(format!("value"), TypeIndication::Any);
        {
            let mut value_write = value.write().unwrap();
            let result = value_write.add_predefined_element(TypedValue::FloatValue(10.0));
            assert!(result.is_ok());
            let result = value_write.add_predefined_element(TypedValue::IntValue(10));
            assert!(result.is_err());
            println!("{}", result.err().unwrap());
        }
    }

    #[test]
    fn serialize_variable_not_evaluated() {
        let value = Variable::new(format!("value"), Some(format!("a")));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#"{"name":"value","exp":"a","is_array":false,"type_indication":"Any"}"#);
    }

}
