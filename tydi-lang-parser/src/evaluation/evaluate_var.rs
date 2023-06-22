use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::evaluation::{evaluate_LogicBit, evaluate_LogicGroup, evaluate_LogicUnion, evaluate_LogicStream, evaluate_expression};
use crate::trait_common::GetName;
use crate::tydi_memory_representation::{IdentifierType, Variable, TypedValue, Scope, EvaluationStatus, TraitCodeLocationAccess, TypeIndication, LogicType, ScopeRelationType, template_args};
use crate::error::TydiLangError;

use super::{Evaluator, evaluate_streamlet, evaluate_impl, evaluate_instance};

pub fn evaluate_value_with_identifier_type(id_name: &String, id_value: TypedValue, id_type: IdentifierType, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    match &id_type {
        IdentifierType::FunctionExp(function_args) => {
            todo!()
        },
        IdentifierType::IndexExp(index_exp) => {
            if let TypedValue::Array(array) = id_value {  //get array value
                let value = evaluate_expression(index_exp.clone(), scope.clone(), evaluator.clone())?;
                if let TypedValue::IntValue(index_int) = value {    //get index value
                    if index_int < 0 {
                        return Err(TydiLangError::new(format!("array index expression {{{}}} is less than 0, array name: {}", index_exp, id_name), crate::tydi_memory_representation::CodeLocation::new_unknown()));
                    }
                    let index_int = index_int as usize;
                    if index_int as usize >= array.len() {
                        return Err(TydiLangError::new(format!("array index out of range, array name: {{{}}}, index value: {}, array size: {}", id_name, value.get_brief_info(), array.len()), crate::tydi_memory_representation::CodeLocation::new_unknown()));
                    }
                    return Ok(array[index_int].clone());
                }
                else {
                    return Err(TydiLangError::new(format!("array index expression {{{}}} is not an integer, value: {}", index_exp, value.get_brief_info()), crate::tydi_memory_representation::CodeLocation::new_unknown()));
                }
            }
            else {
                return Err(TydiLangError::new(format!("identifier {{{}}} is not an array, value: {}", id_name, id_value.get_brief_info()), crate::tydi_memory_representation::CodeLocation::new_unknown()));
            }
        },
        IdentifierType::IdentifierExp => {
            return Ok(id_value);
        },
        _ => unreachable!()
    }
}

pub fn evaluate_id_in_typed_value(id_in_typed_value: TypedValue, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let mut output_value = id_in_typed_value.clone();
    //if the output_value is an identifier
    match &id_in_typed_value {
        TypedValue::Identifier(id) => {
            let id_name = id.read().unwrap().get_id();
            let id_type = id.read().unwrap().get_id_type();
            let id_template_args = id.read().unwrap().get_template_args();
            let id_template_arg_exps = evaluate_template_exps_of_var(&id_template_args, scope.clone(), evaluator.clone())?;
            let (id_var, id_var_scope) = Scope::resolve_identifier(&id_name, &id_template_arg_exps, scope.clone(), ScopeRelationType::resolve_id_default())?;
            let id_typed_value = evaluate_var(id_var.clone(), id_var_scope.clone(), evaluator.clone())?;
            output_value = evaluate_value_with_identifier_type(&id_name, id_typed_value, id_type, scope.clone(), evaluator.clone())?;
        },
        _ => (),
    }

    return Ok(output_value);
}

pub fn evaluate_template_exps_of_var(var_template_expression: &BTreeMap<usize, String>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<Option<BTreeMap<usize, TypedValue>>, TydiLangError> {
    let template_exps = var_template_expression.clone();
    if template_exps.len() == 0 {
        return Ok(None);
    }
    let mut template_values = BTreeMap::new();
    for (arg_index, template_exp) in template_exps {
        let template_exp_value = evaluate_expression(template_exp, scope.clone(), evaluator.clone())?;
        template_values.insert(arg_index, template_exp_value);
    }
    return Ok(Some(template_values));
}

pub fn evaluate_var(var: Arc<RwLock<Variable>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    #[allow(unused_assignments)]
    let mut output_value = TypedValue::UnknwonValue;
    let var_name = var.read().unwrap().get_name();

    //check evaluation status
    let evaluation_status = var.read().unwrap().get_evaluated();
    match evaluation_status {
        EvaluationStatus::NotEvaluated => var.write().unwrap().set_evaluated(EvaluationStatus::EvaluationCount(0)),
        EvaluationStatus::EvaluationCount(i) => {
            var.write().unwrap().set_evaluated(EvaluationStatus::EvaluationCount(i+1));
            if i == 100 {
                let var_read = var.read().unwrap();
                return Err(TydiLangError::new(format!("var {} has been evaluated for {} times, consider mutual reference", var_read.get_name(), i), var_read.get_code_location()));
            }
        },
        EvaluationStatus::Evaluated => {
            let typed_value = var.read().unwrap().get_value();
            return Ok(typed_value)
        },
        EvaluationStatus::Predefined => {
            let typed_value = var.read().unwrap().get_value();
            return Ok(typed_value)
        },
        EvaluationStatus::PreEvaluatedLogicType => (),
    }

    //add evaluation trace
    evaluator.write().unwrap().increase_deepth();
    evaluator.write().unwrap().add_evaluation_trace(var_name.clone(), None, super::EvaluationTraceType::StartEvaluation);

    //if it has template args
    {
        let template_args = var.read().unwrap().get_template_args();
        if template_args.is_some() {
            let template_args = template_args.unwrap();
            if template_args.len() != 0 {
                let mut template_arg_values = BTreeMap::new();
                for (arg_index, arg_exp) in template_args {
                    let value = evaluate_expression(arg_exp, scope.clone(), evaluator.clone())?;
                    template_arg_values.insert(arg_index, value);
                }
                var.write().unwrap().set_template_arg_values(Some(template_arg_values));
            }
        }
    }

    let typed_value = var.read().unwrap().get_value();
    let type_indication = var.read().unwrap().get_type_indication();

    //if this is a ref to another var
    if let TypedValue::RefToVar(inner_var) = typed_value {
        output_value = evaluate_var(inner_var.clone(), scope.clone(), evaluator.clone())?;
        {
            let mut var_write = var.write().unwrap();
            var_write.set_evaluated(EvaluationStatus::Evaluated);
        }
    }

    //if this is a package reference
    else if type_indication == TypeIndication::PackageReference {
        let project = evaluator.read().unwrap().get_project();
        let packages = project.read().unwrap().get_packages();
        let target_package_name = var.read().unwrap().get_exp().unwrap();
        let target_package = packages.get(&target_package_name);
        if target_package.is_none() {
            return Err(TydiLangError::new(format!("no package {} in project", target_package_name), var.read().unwrap().get_code_location()));
        }
        let target_package = target_package.unwrap();
        {
            let mut var_write = var.write().unwrap();
            var_write.set_value(TypedValue::PackageReferenceValue(target_package.clone()));
            var_write.set_evaluated(EvaluationStatus::Evaluated);
        }
        output_value = var.read().unwrap().get_value();
    }

    //if this is a logic type reference
    else if let TypeIndication::LogicTypeRef(logic_ref) = type_indication {
        let mut real_logic_type = evaluate_expression(logic_ref, scope.clone(), evaluator.clone())?;
        //real_logic_type could be evaluated to an identifier
        if let TypedValue::Identifier(identifier) = real_logic_type {
            let id: String = identifier.read().unwrap().get_id();
            let template_args = identifier.read().unwrap().get_template_args();
            let template_exps = evaluate_template_exps_of_var(&template_args, scope.clone(), evaluator.clone())?;
            let (logic_type, logic_type_scope) = Scope::resolve_identifier(&id, &template_exps, scope.clone(), ScopeRelationType::resolve_id_default())?;
            real_logic_type = evaluate_var(logic_type.clone(), logic_type_scope.clone(), evaluator.clone())?;
        }

        {
            let mut var_write = var.write().unwrap();
            var_write.set_value(real_logic_type.clone());
            var_write.set_evaluated(EvaluationStatus::Evaluated);
        }
        output_value = real_logic_type;
    }

    //if this is a logic type and it is pre-evaluated
    else if evaluation_status == EvaluationStatus::PreEvaluatedLogicType && type_indication == TypeIndication::AnyLogicType {
        let self_logic_value = match var.read().unwrap().get_value() {
            TypedValue::LogicTypeValue(v) => v,
            _ => unreachable!()
        };
        let self_logic_value = self_logic_value.read().unwrap();
        match &*self_logic_value {
            LogicType::LogicNullType => {
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(TypedValue::LogicTypeValue(Arc::new(RwLock::new(LogicType::LogicNullType))));
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
                output_value = var.read().unwrap().get_value();
            },
            LogicType::LogicBitType(logic_var) => {
                let value = evaluate_LogicBit(logic_var.clone(), scope.clone(), evaluator.clone())?;
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(value);
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
                output_value = var.read().unwrap().get_value();
            },
            LogicType::LogicGroupType(logic_var) => {
                let value = evaluate_LogicGroup(logic_var.clone(), scope.clone(), evaluator.clone())?;
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(value);
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
                output_value = var.read().unwrap().get_value();
            },
            LogicType::LogicUnionType(logic_var) => {
                let value = evaluate_LogicUnion(logic_var.clone(), scope.clone(), evaluator.clone())?;
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(value);
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
                output_value = var.read().unwrap().get_value();
            },
            LogicType::LogicStreamType(logic_var) => {
                let value = evaluate_LogicStream(logic_var.clone(), scope.clone(), evaluator.clone())?;
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(value);
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
                output_value = var.read().unwrap().get_value();
            },
        }
    }

    //this is a logic type but we don't know exactly which type it is, probably it's a var reference 
    else if type_indication == TypeIndication::AnyLogicType {
        let var_exp = var.read().unwrap().get_exp();
        match var_exp {
            Some(exp) => {  //evaluate the expression
                output_value = evaluate_expression(exp.clone(), scope.clone(), evaluator.clone())?;
                //if the output_value is an identifier
                output_value = evaluate_id_in_typed_value(output_value, scope.clone(), evaluator.clone())?;
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(output_value.clone());
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
            },
            None => {
                //this should be an instance of logic type template
                let value = var.read().unwrap().get_value();
                if let TypedValue::LogicTypeValue(logic_type) = &value {
                    match &*logic_type.read().unwrap() {
                        LogicType::LogicGroupType(group) => {
                            evaluate_LogicGroup(group.clone(), scope.clone(), evaluator.clone())?;
                        },
                        LogicType::LogicUnionType(union) => {
                            evaluate_LogicUnion(union.clone(), scope.clone(), evaluator.clone())?;
                        },
                        _ => return Err(TydiLangError::new(format!("only LogicGroup and LogicUnion can be templates, find {}", logic_type.read().unwrap().get_brief_info()), var.read().unwrap().get_code_location()))
                    }
                };
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
                output_value = value;
            },
        }
    }

    else if type_indication == TypeIndication::AnyStreamlet {
        let value_of_the_var = var.read().unwrap().get_value();
        match &value_of_the_var {
            TypedValue::Streamlet(target_streamlet) => {
                output_value = evaluate_streamlet(target_streamlet.clone(), scope.clone(), evaluator.clone())?;
            },
            TypedValue::UnknwonValue => {       //here we should get the expression of the streamlet
                let streamlet_exp = var.read().unwrap().get_exp();
                match streamlet_exp {
                    Some(streamlet_exp) => {
                        output_value = evaluate_expression(streamlet_exp, scope.clone(), evaluator.clone())?;
                    },
                    None => unreachable!(),
                }
            },
            _ => unreachable!()
        }

        //if the output_value is an identifier
        output_value = evaluate_id_in_typed_value(output_value, scope.clone(), evaluator.clone())?;
        {
            let mut var_write = var.write().unwrap();
            var_write.set_value(output_value.clone());
            var_write.set_evaluated(EvaluationStatus::Evaluated);
        }
    }

    else if type_indication == TypeIndication::AnyImplementation {
        let value_of_the_var = var.read().unwrap().get_value();
        match &value_of_the_var {
            TypedValue::Implementation(target_impl) => {
                output_value = evaluate_impl(target_impl.clone(), scope.clone(), evaluator.clone())?;
                //if the output_value is an identifier
                output_value = evaluate_id_in_typed_value(output_value, scope.clone(), evaluator.clone())?;
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(output_value.clone());
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
            },
            TypedValue::UnknwonValue => {       //here we should get the expression of the implementation
                let impl_exp = var.read().unwrap().get_exp();
                match impl_exp {
                    Some(impl_exp) => {
                        output_value = evaluate_expression(impl_exp, scope.clone(), evaluator.clone())?;
                    },
                    None => unreachable!(),
                }
            },
            _ => unreachable!()
        }

        //if the output_value is an identifier
        output_value = evaluate_id_in_typed_value(output_value, scope.clone(), evaluator.clone())?;
        {
            let mut var_write = var.write().unwrap();
            var_write.set_value(output_value.clone());
            var_write.set_evaluated(EvaluationStatus::Evaluated);
        }
    }

    else if type_indication == TypeIndication::AnyInstance {
        let value_of_the_var = var.read().unwrap().get_value();
        match &value_of_the_var {
            TypedValue::Instance(target_inst) => {
                output_value = evaluate_instance(target_inst.clone(), scope.clone(), evaluator.clone())?;
                //if the output_value is an identifier
                output_value = evaluate_id_in_typed_value(output_value, scope.clone(), evaluator.clone())?;
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(output_value.clone());
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
            },

            _ => unreachable!()
        }
    }

    else if type_indication == TypeIndication::AnyPort {
        let net_exp = var.read().unwrap().get_exp();
        let net_exp = match net_exp {
            Some(net_exp) => {
                net_exp
            }
            None => unreachable!("the parser side should give us the expression")
        };
        output_value = evaluate_expression(net_exp, scope.clone(), evaluator.clone())?;
        output_value = evaluate_id_in_typed_value(output_value, scope.clone(), evaluator.clone())?;
        {
            let mut var_write = var.write().unwrap();
            var_write.set_value(output_value.clone());
            var_write.set_evaluated(EvaluationStatus::Evaluated);
        }
    }

    //we don't know what this variable is, so we evaluate it's expression
    else if type_indication == TypeIndication::Any {
        let var_exp = var.read().unwrap().get_exp();
        match var_exp {
            Some(exp) => {  //evaluate the expression
                output_value = evaluate_expression(exp.clone(), scope.clone(), evaluator.clone())?;
                //if the output_value is an identifier
                output_value = evaluate_id_in_typed_value(output_value, scope.clone(), evaluator.clone())?;
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(output_value.clone());
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
            },
            None => {
                todo!()
            },
        }
    }

    // we know this variable is of a basic type, so we evaluate it's expression
    else if type_indication == TypeIndication::String || type_indication == TypeIndication::Int || type_indication == TypeIndication::Float || type_indication == TypeIndication::Bool {
        let var_exp = var.read().unwrap().get_exp();
        match var_exp {
            Some(exp) => {  //evaluate the expression
                output_value = evaluate_expression(exp.clone(), scope.clone(), evaluator.clone())?;
                //if the output_value is an identifier
                output_value = evaluate_id_in_typed_value(output_value, scope.clone(), evaluator.clone())?;
                {
                    let mut var_write = var.write().unwrap();
                    var_write.set_value(output_value.clone());
                    var_write.set_evaluated(EvaluationStatus::Evaluated);
                }
            },
            None => {
                todo!()
            },
        }
    }

    else {
        todo!()
    }

    evaluator.write().unwrap().add_evaluation_trace(var_name.clone(), Some(output_value.clone()), super::EvaluationTraceType::FinishEvaluation);
    evaluator.write().unwrap().decrease_deepth();
    return Ok(output_value);
}