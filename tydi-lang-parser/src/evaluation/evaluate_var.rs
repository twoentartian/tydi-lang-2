use std::sync::{Arc, RwLock};

use crate::evaluation::{evaluate_LogicBit, evaluate_LogicGroup, evaluate_LogicUnion, evaluate_LogicStream, evaluate_expression};
use crate::trait_common::GetName;
use crate::tydi_memory_representation::{Variable, TypedValue, Scope, EvaluationStatus, TraitCodeLocationAccess, TypeIndication, LogicType, ScopeRelationType};
use crate::error::TydiLangError;

use super::Evaluator;

pub fn evaluate_id_in_typed_value(id_in_typed_value: TypedValue, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let mut output_value = id_in_typed_value.clone();
    //if the output_value is an identifier
    match &id_in_typed_value {
        TypedValue::Identifier(id) => {
            let id_read = id.read().unwrap();
            let (id_var, id_var_scope) = Scope::resolve_identifier(&id_read.id, scope.clone(), ScopeRelationType::resolve_id_default())?;
            let id_typed_value = evaluate_var(id_var.clone(), id_var_scope.clone(), evaluator.clone())?;
            output_value = id_typed_value;
        },
        _ => (),
    }

    return Ok(output_value);
}

pub fn evaluate_var(var: Arc<RwLock<Variable>>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let mut output_value = TypedValue::UnknwonValue;

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
        EvaluationStatus::Evaluated => return Ok(var.read().unwrap().get_value()),
        EvaluationStatus::Predefined => return Ok(var.read().unwrap().get_value()),
        EvaluationStatus::PreEvaluatedLogicType => (),
    }

    //if this is a package reference
    let type_indication = var.read().unwrap().get_type_indication();
    if type_indication == TypeIndication::PackageReference {
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
            let (logic_type, logic_type_scope) = Scope::resolve_identifier(&id, scope.clone(), ScopeRelationType::resolve_id_default())?;
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
                todo!()
            },
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
        unreachable!()
    }

    return Ok(output_value);
}