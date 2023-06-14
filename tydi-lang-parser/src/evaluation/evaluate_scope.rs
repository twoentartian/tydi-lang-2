use std::sync::{Arc, RwLock};
use std::vec;

use crate::deep_clone::DeepClone_ArcLock;
use crate::trait_common::GetName;
use crate::tydi_memory_representation::{Streamlet, Scope, TypedValue, TypeIndication, EvaluationStatus, ScopeType, Implementation, LogicGroup, LogicUnion, TraitCodeLocationAccess, GetScope, If, For, Variable, ScopeRelationType};

use crate::error::TydiLangError;

use super::{Evaluator, evaluate_port, evaluate_instance, evaluate_net, evaluate_var};

#[derive(Clone, Debug)]
pub enum ScopeOwner {
    Streamlet(Arc<RwLock<Streamlet>>),
    Implementation(Arc<RwLock<Implementation>>),
    LogicGroup(Arc<RwLock<LogicGroup>>),
    LogicUnion(Arc<RwLock<LogicUnion>>),
    If(Arc<RwLock<If>>),
    For(Arc<RwLock<For>>),
}

pub fn evaluate_scope(target_scope: Arc<RwLock<Scope>>, scope_type: &ScopeType, scope_owner: &ScopeOwner, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(), TydiLangError> {
    let vars_in_scope = target_scope.read().unwrap().get_variables();
    let mut vars_to_remove = vec![];

    for (var_name, var) in vars_in_scope {
        let var_value = var.read().unwrap().get_value();

        //evaluate if
        if let TypedValue::If(if_target) = &var_value {
            vars_to_remove.push(var_name.clone());

            let if_var = if_target.read().unwrap().get_if_exp();
            let value = evaluate_var(if_var.clone(), target_scope.clone(), evaluator.clone())?;
            let if_exp_value = match &value {
                &TypedValue::BoolValue(v) => v,
                _ => return Err(TydiLangError::new(format!("if expression must be a bool, found {}", value.get_brief_info()), if_var.read().unwrap().get_code_location())),
            };
            let mut scope_to_evaluate = None;
            if if_exp_value {
                scope_to_evaluate = Some(if_target.read().unwrap().get_scope());
            }
            //check elif blocks
            else {
                let elif_blocks = if_target.write().unwrap().get_elif_blocks().clone();
                for i in 0..elif_blocks.len() {
                    let elif_block = &elif_blocks[&i];
                    let elif_var = elif_block.get_elif_exp();
                    let value = evaluate_var(elif_var.clone(), target_scope.clone(), evaluator.clone())?;
                    let elif_exp_value = match &value {
                        &TypedValue::BoolValue(v) => v,
                        _ => return Err(TydiLangError::new(format!("elif expression must be a bool, found {}", value.get_brief_info()), if_var.read().unwrap().get_code_location())),
                    };
                    if elif_exp_value {
                        scope_to_evaluate = Some(elif_block.get_scope());
                    }
                }
            }
            //check else
            if let None = scope_to_evaluate {
                let else_block = if_target.write().unwrap().get_else_block().clone();
                if let Some(else_block) = else_block {
                    scope_to_evaluate = Some(else_block.get_scope());
                }
            }

            //evaluate scope and copy vars to the scope out of "if"
            match scope_to_evaluate {
                Some(scope_to_evaluate) => {
                    evaluate_scope(scope_to_evaluate.clone(), scope_type, scope_owner, scope_to_evaluate.clone(), evaluator.clone())?;
                    let vars_in_if_scope = scope_to_evaluate.read().unwrap().get_variables();
                    for (var_name, var) in vars_in_if_scope {
                        match target_scope.read().unwrap().get_variables().get(&var_name) {
                            Some(external_var) => return Err(TydiLangError::new_multiple_locations(format!("id insides an if block conflicts with an external name"), vec![external_var.read().unwrap().get_code_location(), var.read().unwrap().get_code_location()])), //this variable exists in the external if scope, so we don't override it
                            None => (),
                        }
                        target_scope.write().unwrap().add_var(var.clone())?;
                    }
                },
                None => (), // no else hits, no if & elif hits
            }
        }

        //evaluate for
        if let TypedValue::For(for_target) = &var_value {
            vars_to_remove.push(var_name.clone());

            let for_var_name = for_target.read().unwrap().get_for_var_name();
            let for_array_var = for_target.read().unwrap().get_for_array_exp();
            let for_array_var_value = evaluate_var(for_array_var.clone(), scope.clone(), evaluator.clone())?;
            let for_array_var_value = match for_array_var_value {
                TypedValue::Array(v) => v,
                _ => {
                    return Err(TydiLangError::new(format!("\"for\" range variable is not an array, value:{}", for_array_var_value.get_brief_info()), for_array_var.read().unwrap().get_code_location()));
                }
            };
            let for_scope = for_target.read().unwrap().get_scope();

            let mut for_evaluation_count: usize = 0;
            for single_for_element in for_array_var_value {
                let for_scope_deepcloned = for_scope.read().unwrap().deep_clone_arclock();

                //add var to the deepcloned for scope
                let for_element_var = Variable::new_predefined(for_var_name.clone(), single_for_element);
                for_scope_deepcloned.write().unwrap().add_var(for_element_var)?;

                //evluate scope
                evaluate_scope(for_scope_deepcloned.clone(), scope_type, scope_owner, for_scope_deepcloned.clone(), evaluator.clone())?;

                //copy all variables in this scope (except the "for_element_var") back to origin scope
                let all_vars_in_deepcloned_scope = for_scope_deepcloned.read().unwrap().get_variables();
                for (var_name, var) in all_vars_in_deepcloned_scope {
                    if var_name == for_var_name {
                        continue;   //skip the for variable
                    }

                    evaluate_var(var.clone(), for_scope_deepcloned.clone(), evaluator.clone())?;

                    let var_value = var.read().unwrap().get_value();
                    let result = Scope::resolve_identifier(&var_name, target_scope.clone(), ScopeRelationType::resolve_id_in_current_scope()); //try to find the variable in the target scope
                    let outside_var = match result {
                        Ok((outside_var, _)) => {
                            outside_var
                        },
                        Err(_) => {
                            // we should create an array variable
                            let array_var = Variable::new_predefined_empty_array(var_name.clone(), TypeIndication::Array(Box::new(TypeIndication::Unknown)));
                            target_scope.write().unwrap().add_var(array_var.clone())?;
                            array_var
                        },
                    };

                    let outside_var_type = outside_var.read().unwrap().get_type_indication();
                    match &outside_var_type {
                        TypeIndication::Array(_) => (),
                        _ => return Err(TydiLangError::new_multiple_locations(format!("find an external variable outside of \"for\" scope, but it's not an array. Its type indication:{}", outside_var_type.to_string()),  vec![outside_var.read().unwrap().get_code_location(), var.read().unwrap().get_code_location()])),
                    }
                    let outside_var_value = outside_var.read().unwrap().get_value();
                    let mut existing_array = match &outside_var_value {
                        TypedValue::Array(array) => {
                            array.clone()
                        }
                        _ => return Err(TydiLangError::new_multiple_locations(format!("find an external variable outside of \"for\" scope, but it's not an array. Its value:{}", outside_var_value.get_brief_info()),  vec![outside_var.read().unwrap().get_code_location(), var.read().unwrap().get_code_location()])),
                    };
                    while existing_array.len() < for_evaluation_count {
                        existing_array.push(TypedValue::UnknwonValue);
                    }
                    existing_array.push(var_value);
                    outside_var.write().unwrap().set_value(TypedValue::Array(existing_array));
                }

                for_evaluation_count += 1;
            }


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

    return Ok(());
}