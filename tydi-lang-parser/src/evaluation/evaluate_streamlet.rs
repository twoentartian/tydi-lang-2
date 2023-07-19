use std::sync::{Arc, RwLock};

use crate::tydi_memory_representation::{Streamlet, Scope, TypedValue, GetScope, Port};

use crate::error::TydiLangError;

use super::{Evaluator, evaluate_var, evaluate_scope, ScopeOwner};


pub fn evaluate_streamlet(target: Arc<RwLock<Streamlet>>, _scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let streamlet_scope = target.read().unwrap().get_scope();

    evaluate_scope(streamlet_scope.clone(), &crate::tydi_memory_representation::ScopeType::StreamletScope, &ScopeOwner::Streamlet(target.clone()), streamlet_scope.clone(), evaluator.clone())?;

    return Ok(TypedValue::Streamlet(target));
}

pub fn evaluate_port(port: Arc<RwLock<Port>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let time_domain_var = port.read().unwrap().get_time_domain();
    evaluate_var(time_domain_var.clone(), scope.clone(), evaluator.clone())?;

    let logic_type = port.read().unwrap().get_logical_type();
    evaluate_var(logic_type.clone(), scope.clone(), evaluator.clone())?;

    return Ok(TypedValue::Port(port));
}
