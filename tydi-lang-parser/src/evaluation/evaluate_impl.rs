use std::sync::{Arc, RwLock};

use crate::trait_common::GetName;
use crate::tydi_memory_representation::{Streamlet, Scope, TypedValue, GetScope, Port, EvaluationStatus, Implementation, TraitCodeLocationAccess, Variable, Instance, Net};

use crate::error::TydiLangError;

use super::{Evaluator, evaluate_var};


pub fn evaluate_impl(target: Arc<RwLock<Implementation>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let target_derived_streamlet = target.read().unwrap().get_derived_streamlet();
    let derived_streamlet = evaluate_var(target_derived_streamlet.clone(), scope.clone(), evaluator.clone())?;
    match &derived_streamlet {
        TypedValue::Streamlet(_) => (),
        _ => return Err(TydiLangError::new(format!("{} is not a streamlet, but used in defining impl({})", target_derived_streamlet.read().unwrap().get_name(), target.read().unwrap().get_name()), target_derived_streamlet.read().unwrap().get_code_location()))
    }

    //add self variable
    let impl_scope = target.read().unwrap().get_scope();
    let self_var = Variable::new_builtin(format!("self"), TypedValue::RefToVar(target_derived_streamlet.clone()));
    {
        let mut impl_scope_write = impl_scope.write().unwrap();
        impl_scope_write.add_var(self_var)?;
    }

    //for impl, we only evaluate instances and connections
    let variables = impl_scope.read().unwrap().get_variables();
    //evaluate instances first
    for (_, var) in &variables {
        let var_type = var.read().unwrap().get_type_indication();
        match &var_type {
            crate::tydi_memory_representation::TypeIndication::AnyInstance => (),
            _ => continue,  //skip this variable
        }

        let instance_typed_value = var.read().unwrap().get_value();
        let instance = match &instance_typed_value {
            TypedValue::Instance(inst) => inst.clone(),
            _ => unreachable!("something wrong on the parser side, the value should be an instance")
        };
        {
            let mut instance_write = instance.write().unwrap();
            instance_write.set_parent_impl(Some(target.clone()));
        }
        let output_value = evaluate_instance(instance.clone(), impl_scope.clone(), evaluator.clone())?;
        {
            let mut var_write = var.write().unwrap();
            var_write.set_value(output_value.clone());
            var_write.set_evaluated(EvaluationStatus::Evaluated);
        }
    }

    //then evaluate nets
    for (_, var) in &variables {
        let var_type = var.read().unwrap().get_type_indication();
        match &var_type {
            crate::tydi_memory_representation::TypeIndication::AnyNet => (),
            _ => continue,  //skip this variable
        }

        let net_typed_value = var.read().unwrap().get_value();
        let net = match &net_typed_value {
            TypedValue::Net(net) => net.clone(),
            _ => unreachable!("something wrong on the parser side, the value should be a net")
        };
        {
            let mut net_write = net.write().unwrap();
            net_write.set_parent_impl(Some(target.clone()));
        }
        let output_value = evaluate_net(net.clone(), impl_scope.clone(), evaluator.clone())?;
        {
            let mut var_write = var.write().unwrap();
            var_write.set_value(output_value.clone());
            var_write.set_evaluated(EvaluationStatus::Evaluated);
        }
    }

    return Ok(TypedValue::Implementation(target.clone()));
}


pub fn evaluate_instance(target: Arc<RwLock<Instance>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let target_derived_impl = target.read().unwrap().get_derived_implementation();
    let derived_impl = evaluate_var(target_derived_impl.clone(), scope.clone(), evaluator.clone())?;
    match &derived_impl {
        TypedValue::Implementation(_) => (),
        _ => return Err(TydiLangError::new(format!("{} is not an implementation, but used in defining instance({})", target_derived_impl.read().unwrap().get_name(), target.read().unwrap().get_name()), target_derived_impl.read().unwrap().get_code_location()))
    }

    return Ok(TypedValue::Instance(target.clone()));
}


pub fn evaluate_net(target: Arc<RwLock<Net>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    //evaluate lhs
    let lhs_port_var = target.read().unwrap().get_source();
    let lhs_port_value = evaluate_var(lhs_port_var.clone(), scope.clone(), evaluator.clone())?;
    let lhs_port = match &lhs_port_value {
        TypedValue::Port(port) => port.clone(),
        _ => return Err(TydiLangError::new(format!("{} is not a port, but used in defining net {}", lhs_port_var.read().unwrap().get_name(), target.read().unwrap().get_name()), target.read().unwrap().get_code_location())),
    };

    let rhs_port_var = target.read().unwrap().get_sink();
    let rhs_port_value = evaluate_var(rhs_port_var.clone(), scope.clone(), evaluator.clone())?;
    let rhs_port = match &rhs_port_value {
        TypedValue::Port(port) => port.clone(),
        _ => return Err(TydiLangError::new(format!("{} is not a port, but used in defining net {}", lhs_port_var.read().unwrap().get_name(), target.read().unwrap().get_name()), target.read().unwrap().get_code_location())),
    };

    {
        let mut target_write = target.write().unwrap();
        target_write.set_source_port(Some(lhs_port.clone()));
        target_write.set_sink_port(Some(rhs_port.clone()));
    }

    //todo set src/sink component
    todo!()

    return Ok(TypedValue::Net(target.clone()));
}