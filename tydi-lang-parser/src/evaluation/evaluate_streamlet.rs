use std::sync::{Arc, RwLock};

use crate::tydi_memory_representation::{Streamlet, Scope, TypedValue, GetScope, Port, EvaluationStatus};

use crate::error::TydiLangError;

use super::{Evaluator, evaluate_var};


pub fn evaluate_streamlet(target: Arc<RwLock<Streamlet>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let streamlet_scope = target.read().unwrap().get_scope();

    //for streamlet, we only evaluate ports
    let variables = streamlet_scope.read().unwrap().get_variables();
    for (_, var) in variables {
        let var_type = var.read().unwrap().get_type_indication();
        match &var_type {
            crate::tydi_memory_representation::TypeIndication::AnyPort => (),
            _ => continue,  //skip this variable
        }

        let port_typed_value = var.read().unwrap().get_value();
        let port = match &port_typed_value {
            TypedValue::Port(port) => port.clone(),
            _ => unreachable!("something wrong on the parser side, the value should be a port")
        };
        {
            let mut port_write = port.write().unwrap();
            port_write.set_parent_streamlet(Some(target.clone()));
        }
        let output_value = evaluate_port(port.clone(), streamlet_scope.clone(), evaluator.clone())?;
        {
            let mut var_write = var.write().unwrap();
            var_write.set_value(output_value.clone());
            var_write.set_evaluated(EvaluationStatus::Evaluated);
        }
    }

    return Ok(TypedValue::Streamlet(target));
}

pub fn evaluate_port(port: Arc<RwLock<Port>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let time_domain_var = port.read().unwrap().get_time_domain();
    evaluate_var(time_domain_var.clone(), scope.clone(), evaluator.clone())?;

    let logic_type = port.read().unwrap().get_logical_type();
    evaluate_var(logic_type.clone(), scope.clone(), evaluator.clone())?;

    return Ok(TypedValue::Port(port));
}
