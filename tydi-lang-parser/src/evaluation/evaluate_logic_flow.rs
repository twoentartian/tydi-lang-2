use std::sync::{Arc, RwLock};

use crate::deep_clone::DeepClone_ArcLock;
use crate::error::TydiLangError;
use crate::trait_common::GetName;
use crate::tydi_memory_representation::{For, ScopeType, Scope, TypedValue, GetScope, Variable, TypeIndication, TraitCodeLocationAccess, ScopeRelationType, If, CodeLocation};

use crate::evaluation::{evaluate_var, evaluate_scope, ScopeOwner, Evaluator};


pub fn evaluate_for(for_target: Arc<RwLock<For>>, parent_scope: Arc<RwLock<Scope>>, scope_type: &ScopeType, scope_owner: &ScopeOwner, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(), TydiLangError> {
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
            let result = Scope::resolve_identifier(&var_name, &None, &CodeLocation::new_unknown(), parent_scope.clone(), parent_scope.clone(), ScopeRelationType::resolve_id_in_current_scope(), evaluator.clone()); //try to find the variable in the target scope
            let outside_var = match result {
                Ok((outside_var, _)) => {
                    outside_var
                },
                Err(_) => {
                    // we should create an array variable
                    let array_var = Variable::new_predefined_empty_array(var_name.clone(), TypeIndication::Array(Box::new(TypeIndication::Unknown)));
                    parent_scope.write().unwrap().add_var(array_var.clone())?;
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
            //rename the new var value, if possible
            match &var_value {
                TypedValue::UnknwonValue => (),
                TypedValue::Null => (),
                TypedValue::PackageReferenceValue(_) => unreachable!(),
                TypedValue::IntValue(_) => (),
                TypedValue::StringValue(_) => (),
                TypedValue::BoolValue(_) => (),
                TypedValue::FloatValue(_) => (),
                TypedValue::ClockDomainValue(_) => (),
                TypedValue::LogicTypeValue(logic_type) => {
                    let logic_type = logic_type.clone();
                    let logic_type = &*logic_type.read().unwrap();
                    match logic_type {
                        crate::tydi_memory_representation::LogicType::LogicNullType => (),
                        crate::tydi_memory_representation::LogicType::LogicBitType(bit) => {
                            let current_name = bit.read().unwrap().get_name();
                            bit.write().unwrap().set_name(format!("{}_for{}", current_name, for_evaluation_count));
                        },
                        crate::tydi_memory_representation::LogicType::LogicGroupType(_) => unreachable!(),
                        crate::tydi_memory_representation::LogicType::LogicUnionType(_) => unreachable!(),
                        crate::tydi_memory_representation::LogicType::LogicStreamType(stream) => {
                            let current_name = stream.read().unwrap().get_name();
                            stream.write().unwrap().set_name(format!("{}_for{}", current_name, for_evaluation_count));
                        },
                    }
                },
                TypedValue::Streamlet(_) => unreachable!(),
                TypedValue::Port(port) => {
                    let current_name = port.read().unwrap().get_name();
                    port.write().unwrap().set_name(format!("{}_for{}", current_name, for_evaluation_count));
                    port.write().unwrap().set_id_in_scope(Some(format!("{}_for{}", current_name, for_evaluation_count)));
                },
                TypedValue::Implementation(_) => unreachable!(),
                TypedValue::Instance(inst) => {
                    let current_name = inst.read().unwrap().get_name();
                    inst.write().unwrap().set_name(format!("{}_for{}", current_name, for_evaluation_count));
                    inst.write().unwrap().set_id_in_scope(Some(format!("{}_for{}", current_name, for_evaluation_count)));
                },
                TypedValue::Net(net) => {
                    let current_name = net.read().unwrap().get_name();
                    net.write().unwrap().set_name(format!("{}_for{}", current_name, for_evaluation_count));
                },
                TypedValue::If(target_if) => {
                    let current_name = target_if.read().unwrap().get_name();
                    target_if.write().unwrap().set_name(format!("{}_for{}", current_name, for_evaluation_count));
                },
                TypedValue::For(target_for) => {
                    let current_name = target_for.read().unwrap().get_name();
                    target_for.write().unwrap().set_name(format!("{}_for{}", current_name, for_evaluation_count));
                },
                TypedValue::Array(_) => (),
                TypedValue::Function(_) => (),
                TypedValue::RefToVar(_) => (),
                TypedValue::Identifier(_) => unreachable!(),
            }
            existing_array.push(var_value);
            outside_var.write().unwrap().set_value(TypedValue::Array(existing_array));
        }

        //post scope expansion check
        crate::post_compile::check_assert::check_scope(for_scope_deepcloned.clone(), evaluator.clone())?;

        for_evaluation_count += 1;
    }
    return Ok(());
}


pub fn evaluate_if(if_target: Arc<RwLock<If>>, parent_scope: Arc<RwLock<Scope>>, scope_type: &ScopeType, scope_owner: &ScopeOwner, _scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(), TydiLangError> {
    let if_var = if_target.read().unwrap().get_if_exp();
    let value = evaluate_var(if_var.clone(), parent_scope.clone(), evaluator.clone())?;
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
            let value = evaluate_var(elif_var.clone(), parent_scope.clone(), evaluator.clone())?;
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
                match parent_scope.read().unwrap().get_variables().get(&var_name) {
                    Some(external_var) => return Err(TydiLangError::new_multiple_locations(format!("id insides an if block conflicts with an external name"), vec![external_var.read().unwrap().get_code_location(), var.read().unwrap().get_code_location()])), //this variable exists in the external if scope, so we don't override it
                    None => (),
                }
                parent_scope.write().unwrap().add_var(var.clone())?;
            }
        },
        None => (), // no else hits, no if & elif hits
    }

    return Ok(());
}
