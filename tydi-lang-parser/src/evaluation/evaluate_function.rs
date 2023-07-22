use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::evaluation::predefined_function::get_function;
use crate::tydi_memory_representation::{Function, Scope, TypedValue, ScopeRelationType};
use crate::evaluation::{Evaluator, evaluate_expression, evaluate_id_in_typed_value};

pub trait FunctionTrait {
    fn get_id() -> String where Self: Sized;

    fn execute(&self, function: Arc<RwLock<Function>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError>;
}

pub fn evaluate_function(function: Arc<RwLock<Function>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let function_instance = get_function(function.clone())?;
    let value = function_instance.execute(function.clone(), scope.clone(), evaluator.clone())?;

    return Ok(value);
}



