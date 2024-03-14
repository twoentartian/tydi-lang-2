use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::evaluation::{FunctionTrait, Evaluator, evaluate_expression, evaluate_id_in_typed_value};
use crate::tydi_memory_representation::{TypedValue, Scope, Function, ScopeRelationType, TraitCodeLocationAccess};
use crate::error::TydiLangError;

pub fn get_function(function: Arc<RwLock<Function>>) -> Result<Box<dyn FunctionTrait>, TydiLangError> {
    let function_id = function.read().unwrap().get_function_id();
    
    if function_id == FunctionAssert::get_id() {
        return Ok(Box::new(FunctionAssert{}));
    }

    if function_id == FunctionToString::get_id() {
        return Ok(Box::new(FunctionToString{}));
    }

    return Err(TydiLangError::new(format!("unknown function {} ", function_id), function.read().unwrap().get_code_location()));
}


struct FunctionAssert {}

impl FunctionAssert {
    const ID: &'static str = "assert";
}

impl FunctionTrait for FunctionAssert {
    fn get_id() -> String where Self: Sized {
        return String::from(FunctionAssert::ID);
    }

    fn execute(&self, function: Arc<RwLock<Function>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
        let function_args = function.read().unwrap().get_function_arg_exps();
        let mut function_arg_values = BTreeMap::new();
        for index in 0 .. function_args.len() {
            let function_arg = function_args.get(&index).expect("wrong function arg index");
            let arg_value = evaluate_expression(function_arg.clone(), Some(function.read().unwrap().get_code_location()), scope.clone(), evaluator.clone())?;
            let arg_value = evaluate_id_in_typed_value(arg_value, Some(function.read().unwrap().get_code_location()), ScopeRelationType::resolve_id_default(), scope.clone(), evaluator.clone())?;
            function_arg_values.insert(index, arg_value);
        }
        
        if function_args.len() == 1 || function_args.len() == 2 {
            let assert_exp = function_arg_values.get(&0).expect("bug: arg not on index 0");
            match assert_exp {
                TypedValue::BoolValue(assert_exp) => {
                    if *assert_exp {
                        return Ok(TypedValue::Null);
                    }
                    else {
                        if function_args.len() == 1 {
                            return Err(TydiLangError::new(format!("assertation failed"), function.read().unwrap().get_code_location()));
                        }
                        else {
                            let message = function_arg_values.get(&1).expect("bug: arg not on index 1");
                            match message {
                                TypedValue::StringValue(message) => {
                                    return Err(TydiLangError::new(format!("assertation failed, message: {}", message), function.read().unwrap().get_code_location()));
                                },
                                _ => return Err(TydiLangError::new(format!("assert message must be a string, get {}", message.get_brief_info()), function.read().unwrap().get_code_location())),
                            }
                        }
                    }
                },
                _ => return Err(TydiLangError::new(format!("assert expression must be a bool, get {}", assert_exp.get_brief_info()), function.read().unwrap().get_code_location())),
            }
        }
        else {
            return Err(TydiLangError::new(format!("assert function has 1 or 2 arguments, get {}", function_args.len()), function.read().unwrap().get_code_location()));
        }
    }
}

struct FunctionToString {}

impl FunctionToString {
    const ID: &str = "toString";
}

impl FunctionTrait for FunctionToString {
    fn get_id() -> String where Self: Sized {
        return String::from(FunctionToString::ID);
    }

    fn execute(&self, function: Arc<RwLock<Function>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
        let function_args = function.read().unwrap().get_function_arg_exps();
        let mut function_arg_values = BTreeMap::new();
        for index in 0 .. function_args.len() {
            let function_arg = function_args.get(&index).expect("wrong function arg index");
            let arg_value = evaluate_expression(function_arg.clone(), Some(function.read().unwrap().get_code_location()), scope.clone(), evaluator.clone())?;
            let arg_value = evaluate_id_in_typed_value(arg_value, Some(function.read().unwrap().get_code_location()), ScopeRelationType::resolve_id_default(), scope.clone(), evaluator.clone())?;
            function_arg_values.insert(index, arg_value);
        }
        
        if function_args.len() == 1 {
            let arg_value = function_arg_values.get(&0).expect("bug: arg not on index 0");
            match arg_value {
                TypedValue::IntValue(v) => return Ok(TypedValue::StringValue(v.to_string())),
                TypedValue::StringValue(v) => return Ok(TypedValue::StringValue(v.to_string())),
                TypedValue::BoolValue(v) => return Ok(TypedValue::StringValue(v.to_string())),
                TypedValue::FloatValue(v) => return Ok(TypedValue::StringValue(v.to_string())),
                TypedValue::ClockDomainValue(v) => return Ok(TypedValue::StringValue(v.to_string())),
                _ => return Err(TydiLangError::new(format!("{} argument has value {}, which is not able to be converted to String", function.read().unwrap().get_function_id(), arg_value.get_brief_info()), function.read().unwrap().get_code_location()))
            }


        }
        else {
            return Err(TydiLangError::new(format!("toString function has 1 argument, get {}", function_args.len()), function.read().unwrap().get_code_location()));
        }
    }
}