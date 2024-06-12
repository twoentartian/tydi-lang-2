use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize, Serializer, Deserialize};

use crate::deep_clone::{DeepClone, DeepClone_ArcLock};
use crate::tydi_memory_representation::{TypedValue, TypeIndication, CodeLocation, TraitCodeLocationAccess, Streamlet, LogicType, Port, Implementation, Instance};
use crate::trait_common::GetName;
use crate::{generate_get_pub, generate_access_pub, generate_set_pub, generate_access, generate_set, generate_get, generate_name};

use super::{Net, Scope, GlobalIdentifier};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EvaluationStatus {
    NotEvaluated,
    EvaluationCount(u32),
    Evaluated,
    Predefined,

    PreEvaluatedLogicType,
}

impl DeepClone for EvaluationStatus {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl EvaluationStatus {
    pub fn is_value_known(&self) -> bool {
        if *self == EvaluationStatus::Evaluated || *self == EvaluationStatus::Predefined {
            return true;
        }
        return false;
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    name: String,
    exp: Option<String>,
    evaluated: EvaluationStatus,
    value: TypedValue,     //the variable can be an array
    array_size: Option<Arc<RwLock<Variable>>>,
    type_indication: TypeIndication,
    is_property_of_scope: bool,
    declare_location: CodeLocation,
    parent_scope: Option<Arc<RwLock<Scope>>>,
    id_in_scope: Option<String>,
    is_name_user_defined: bool,
    template_args: Option<BTreeMap<usize, String>>,
    template_arg_values: Option<BTreeMap<usize, TypedValue>>,
}

impl GetName for Variable {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl DeepClone for Variable {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            exp: self.exp.deep_clone(),
            evaluated: self.evaluated.deep_clone(),
            value: self.value.deep_clone(),
            array_size: self.array_size.deep_clone(),
            type_indication: self.type_indication.deep_clone(),
            is_property_of_scope: self.is_property_of_scope.deep_clone(),
            declare_location: self.declare_location.deep_clone(),
            parent_scope: self.parent_scope.clone(),    //clone should be enough here
            id_in_scope: self.id_in_scope.deep_clone(),
            is_name_user_defined: self.is_name_user_defined.deep_clone(),
            template_args: self.template_args.deep_clone(),
            template_arg_values: self.template_arg_values.deep_clone(),
        };
        return output;
    }
}

impl DeepClone_ArcLock for Variable {
    fn deep_clone_arclock(&self) -> Arc<RwLock<Self>> {
        return Arc::new(RwLock::new(self.deep_clone()));
    }
}

impl Serialize for Variable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        use serde::ser::SerializeStruct;
        if self.evaluated == EvaluationStatus::Evaluated || self.evaluated == EvaluationStatus::Predefined {
            TypedValue::serialize(&self.value, serializer)

            // if self.is_array {
            //     let mut seq = serializer.serialize_seq(Some(self.value.len()))?;
            //     for value in &self.value {
            //         seq.serialize_element(&value)?;
            //     }
            //     seq.end()
            // }
            // else {
            //     let value = &self.value[0];
            //     value.serialize(serializer)
            // }
        }
        else {
            let mut state = serializer.serialize_struct("Variable", 14)?;
            state.serialize_field("name", &self.name)?;
            state.serialize_field("exp", &self.exp)?;
            state.serialize_field("value", &self.value)?;
            state.serialize_field("evaluated", &self.evaluated)?;
            if self.array_size.is_some() {
                let array_size_var = &self.array_size.as_ref().unwrap();
                state.serialize_field("array_size", &*array_size_var.read().unwrap())?;
            }
            state.serialize_field("type_indication", &self.type_indication)?;
            state.serialize_field("is_property_of_scope", &self.is_property_of_scope)?;
            state.serialize_field("declare_location", &self.declare_location)?;
            match &self.parent_scope {
                Some(parent_scope) => {
                    state.serialize_field("parent_scope", &parent_scope.read().unwrap().get_name())?;
                },
                None => {
                    state.serialize_field("parent_scope", "none")?;
                },
            }
            state.serialize_field("id_in_scope", &self.id_in_scope)?;
            state.serialize_field("is_name_user_defined", &self.is_name_user_defined)?;
            state.serialize_field("template_args", &self.template_args)?;
            state.serialize_field("template_arg_values", &self.template_arg_values)?;
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
            value: TypedValue::UnknwonValue,
            array_size: None,
            type_indication: TypeIndication::Any,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_ref(name: String, raw_var: Arc<RwLock<Variable>>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::Evaluated,
            value: TypedValue::RefToVar(raw_var.clone()),
            array_size: None,
            type_indication: TypeIndication::Any,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_with_type_indication(name: String, exp: Option<String>, type_indication: TypeIndication) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            exp: exp,
            evaluated: EvaluationStatus::NotEvaluated,
            value: TypedValue::UnknwonValue,
            array_size: None,
            type_indication: type_indication,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_place_holder() -> Arc<RwLock<Self>> {
        let output = Self {
            name: generate_name::generate_init_value(),
            exp: Some(generate_name::generate_init_value()),
            evaluated: EvaluationStatus::NotEvaluated,
            value: TypedValue::UnknwonValue,
            array_size: None,
            type_indication: TypeIndication::Unknown,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_logic_type(name: String, logic_type: Arc<RwLock<LogicType>>) -> Arc<RwLock<Self>> {
        let typed_value = TypedValue::LogicTypeValue(logic_type);
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::PreEvaluatedLogicType,
            value: typed_value,
            array_size: None,
            type_indication: TypeIndication::AnyLogicType,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_streamlet(name: String, streamlet: Arc<RwLock<Streamlet>>) -> Arc<RwLock<Self>> {
        let typed_value = TypedValue::Streamlet(streamlet);
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::NotEvaluated,
            value: typed_value,
            array_size: None,
            type_indication: TypeIndication::AnyStreamlet,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_port(name: String, port: Arc<RwLock<Port>>) -> Arc<RwLock<Self>> {
        let typed_value = TypedValue::Port(port);
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::NotEvaluated,
            value: typed_value,
            array_size: None,
            type_indication: TypeIndication::AnyPort,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_implementation(name: String, implementation: Arc<RwLock<Implementation>>) -> Arc<RwLock<Self>> {
        let typed_value = TypedValue::Implementation(implementation);
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::NotEvaluated,
            value: typed_value,
            array_size: None,
            type_indication: TypeIndication::AnyImplementation,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_instance(name: String, instance: Arc<RwLock<Instance>>) -> Arc<RwLock<Self>> {
        let typed_value = TypedValue::Instance(instance);
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::NotEvaluated,
            value: typed_value,
            array_size: None,
            type_indication: TypeIndication::AnyInstance,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_net(name: String, net: Arc<RwLock<Net>>) -> Arc<RwLock<Self>> {
        let typed_value = TypedValue::Net(net);
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::NotEvaluated,
            value: typed_value,
            array_size: None,
            type_indication: TypeIndication::AnyNet,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_builtin(name: String, value: TypedValue) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::NotEvaluated,
            value: value.clone(),
            array_size: None,
            type_indication: TypeIndication::infer_from_typed_value(&value),
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_predefined(name: String, value: TypedValue) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::Predefined,
            value: value.clone(),
            array_size: None,
            type_indication: TypeIndication::infer_from_typed_value(&value),
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_predefined_empty_array(name: String, type_indication: TypeIndication) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            exp: None,
            evaluated: EvaluationStatus::Predefined,
            value: TypedValue::Array(vec![]),
            array_size: None,
            type_indication: type_indication,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn add_predefined_element(&mut self, value: TypedValue) -> Result<&mut Self, String> {
        match &self.value {
            TypedValue::Array(_) => (),
            _ => return Err(format!("{} is not an array type", &self.name)),
        }

        //change the type indicator?
        match &mut self.value {
            TypedValue::Array(values) => {
                if values.len() == 0 {
                    if self.type_indication == TypeIndication::Any {
                        self.type_indication = TypeIndication::Array(Box::new(TypeIndication::infer_from_typed_value(&value)));
                    }
                    else {
                        let new_type_indication = TypeIndication::infer_from_typed_value(&value);
                        if self.type_indication != new_type_indication {
                            return Err(format!("{} is inserted to an array of pre-defined type of {}", &self.name, self.type_indication.to_string()));
                        }
                    }
                }
                else {
                    let new_type_indication = TypeIndication::infer_from_typed_value(&value);
                    if self.type_indication != new_type_indication {
                        self.type_indication = TypeIndication::Array(Box::new(TypeIndication::Any));
                    }
                }
                values.push(value);
            },
            _ => unreachable!("value must be an array")
        }
        return Ok(self);
    }

    pub fn new_function_var(name: String, function_exp: String) -> Arc<RwLock<Variable>> {
        let output = Self {
            name: name,
            exp: Some(function_exp),
            evaluated: EvaluationStatus::NotEvaluated,
            value: TypedValue::UnknwonValue,
            array_size: None,
            type_indication: TypeIndication::Function,
            is_property_of_scope: false,
            declare_location: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            is_name_user_defined: false,
            template_args: None,
            template_arg_values: None,
        };
        return Arc::new(RwLock::new(output));
    }

    generate_set_pub!(name, String, set_name);
    generate_access_pub!(exp, Option<String>, get_exp, set_exp);
    generate_access_pub!(value, TypedValue, get_value, set_value);
    generate_access_pub!(type_indication, TypeIndication, get_type_indication, set_type_indication);
    generate_access_pub!(array_size, Option<Arc<RwLock<Variable>>>, get_array_size, set_array_size);
    generate_access_pub!(is_property_of_scope, bool, get_is_property_of_scope, set_is_property_of_scope);
    generate_access_pub!(evaluated, EvaluationStatus, get_evaluated, set_evaluated);
    generate_access_pub!(template_args, Option<BTreeMap<usize, String>>, get_template_args, set_template_args);
    generate_access_pub!(template_arg_values, Option<BTreeMap<usize, TypedValue>>, get_template_arg_values, set_template_arg_values);
    generate_access_pub!(is_name_user_defined, bool, get_is_name_user_defined, set_is_name_user_defined);
}

impl GlobalIdentifier for Variable {
    generate_access!(parent_scope, Option<Arc<RwLock<Scope>>>, get_parent_scope, set_parent_scope);

    generate_access!(id_in_scope, Option<String>, get_id_in_scope, set_id_in_scope);
}

#[cfg(test)]
mod test_var {
    use super::*;

    #[test]
    fn serialize_variable() {
        let value = Variable::new_predefined(format!("value"), TypedValue::BoolValue(true));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#"{"type":"BoolValue","value":true}"#);

        let value = Variable::new_predefined(format!("value"), TypedValue::IntValue(100));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#"{"type":"IntValue","value":100}"#);

        let value = Variable::new_predefined(format!("value"), TypedValue::StringValue(format!("123")));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#"{"type":"StringValue","value":"123"}"#);

        let value = Variable::new_predefined(format!("value"), TypedValue::FloatValue(99.99));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#"{"type":"FloatValue","value":99.99}"#);

        let value = Variable::new_predefined(format!("value"), TypedValue::ClockDomainValue(format!("test_clock_domain")));
        let json_output = serde_json::to_string(&*value.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        assert_eq!(json_output, r#"{"type":"ClockDomainValue","value":"!CLOCK_test_clock_domain"}"#);
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
        assert_eq!(json_output, r#"{"type":"Array","value":[{"type":"IntValue","value":10},{"type":"IntValue","value":15},{"type":"IntValue","value":20}]}"#);

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
            assert!(result.is_ok());
        }
    }

}
