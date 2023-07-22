use std::sync::{Arc, RwLock};
use std::vec;

use crate::tydi_memory_representation::{Streamlet, Scope, TypedValue, TypeIndication, EvaluationStatus, ScopeType, Implementation, LogicGroup, LogicUnion};

use crate::error::TydiLangError;

use crate::evaluation::{Evaluator, evaluate_port, evaluate_instance, evaluate_net, evaluate_var, evaluate_for};

use super::evaluate_if;

#[derive(Clone, Debug)]
pub enum ScopeOwner {
    Streamlet(Arc<RwLock<Streamlet>>),
    Implementation(Arc<RwLock<Implementation>>),
    LogicGroup(Arc<RwLock<LogicGroup>>),
    LogicUnion(Arc<RwLock<LogicUnion>>),
}

pub fn evaluate_scope(target_scope: Arc<RwLock<Scope>>, scope_type: &ScopeType, scope_owner: &ScopeOwner, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(), TydiLangError> {
    let vars_in_scope = target_scope.read().unwrap().get_variables();
    let mut vars_to_remove = vec![];

    for (var_name, var) in vars_in_scope {
        let var_value = var.read().unwrap().get_value();

        //evaluate if
        if let TypedValue::If(if_target) = &var_value {
            vars_to_remove.push(var_name.clone());

            evaluate_if(if_target.clone(), target_scope.clone(), scope_type, scope_owner, scope.clone(), evaluator.clone())?;
        }

        //evaluate for
        if let TypedValue::For(for_target) = &var_value {
            vars_to_remove.push(var_name.clone());

            evaluate_for(for_target.clone(), target_scope.clone(), scope_type, scope_owner, scope.clone(), evaluator.clone())?;
        }
    }

    //remove if / for variables
    let mut target_scope_vars =  target_scope.read().unwrap().get_variables();
    for var_name in vars_to_remove {
        target_scope_vars.remove(&var_name);
    }
    target_scope.write().unwrap().set_variables(target_scope_vars);



    //evaluate scope-specific variables
    match scope_type {
        ScopeType::GroupScope => {
            let logic_group_scope = &target_scope;
            match &scope_owner {
                ScopeOwner::LogicGroup(_) => (),
                _ => unreachable!(),
            };

            let vars_in_scope = logic_group_scope.read().unwrap().get_variables();
            for (_, var) in vars_in_scope {
                let property_of_scope = var.read().unwrap().get_is_property_of_scope();
                if !property_of_scope { continue; }
                evaluate_var(var.clone(), scope.clone(), evaluator.clone())?;
            }
        },
        ScopeType::UnionScope => {
            let logic_union_scope = &target_scope;
            match &scope_owner {
                ScopeOwner::LogicUnion(_) => (),
                _ => unreachable!(),
            };

            let vars_in_scope = logic_union_scope.read().unwrap().get_variables();
            for (_, var) in vars_in_scope {
                let property_of_scope = var.read().unwrap().get_is_property_of_scope();
                if !property_of_scope { continue; }
                evaluate_var(var.clone(), scope.clone(), evaluator.clone())?;
            }
        },
        ScopeType::StreamletScope => {
            let streamlet_scope = &target_scope;
            let parent_streamlet = match &scope_owner {
                ScopeOwner::Streamlet(v) => v,
                _ => unreachable!(),
            };
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
                    port_write.set_parent_streamlet(Some(parent_streamlet.clone()));
                }
                let output_value = evaluate_port(port.clone(), scope.clone(), evaluator.clone())?;
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(output_value.clone());
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
            }
        },
        ScopeType::ImplementationScope => {
            let implementation_scope = &target_scope;
            let parent_implementation = match &scope_owner {
                ScopeOwner::Implementation(v) => v,
                _ => unreachable!(),
            };

            //for impl, we first evaluate instances and nets
            let variables = implementation_scope.read().unwrap().get_variables();
            for (var_name, var) in &variables {
                if var_name == "self" {continue;}   //skip self instance
                let var_type = var.read().unwrap().get_type_indication();
                match &var_type {
                    TypeIndication::AnyInstance => {
                        let instance_typed_value = var.read().unwrap().get_value();
                        let instance = match &instance_typed_value {
                            TypedValue::Instance(inst) => inst.clone(),
                            _ => unreachable!("something wrong on the parser side, the value should be an instance")
                        };
                        let output_value = evaluate_instance(instance.clone(), scope.clone(), evaluator.clone())?;
                        {
                            let mut var_write = var.write().unwrap();
                            var_write.set_value(output_value.clone());
                            var_write.set_evaluated(EvaluationStatus::Evaluated);
                        }
                    },
                    TypeIndication::AnyNet => {
                        let net_typed_value = var.read().unwrap().get_value();
                        let net = match &net_typed_value {
                            TypedValue::Net(net) => net.clone(),
                            _ => unreachable!("something wrong on the parser side, the value should be a net")
                        };
                        {
                            let mut net_write = net.write().unwrap();
                            net_write.set_parent_impl(Some(parent_implementation.clone()));
                        }
                        let output_value = evaluate_net(net.clone(), scope.clone(), evaluator.clone())?;
                        {
                            let mut var_write = var.write().unwrap();
                            var_write.set_value(output_value.clone());
                            var_write.set_evaluated(EvaluationStatus::Evaluated);
                        }
                    }
                    _ => (),  //skip this variable
                }
            }
        },
        _ => unreachable!()
    }


    //post evaluation steps
    //assertion
    crate::post_compile::check_assert::check_scope(target_scope.clone(), evaluator.clone())?;

    return Ok(());
}