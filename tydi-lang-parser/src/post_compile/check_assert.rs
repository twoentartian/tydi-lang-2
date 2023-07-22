use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::tydi_memory_representation::{Scope, Project, GetScope};
use crate::evaluation::{Evaluator, evaluate_var, EvaluationTrace};

pub fn check_scope(scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(), TydiLangError> {
    let all_vars = scope.read().unwrap().get_variables();
    for (_, var) in all_vars {
        let var_type_indication = var.read().unwrap().get_type_indication();
        match var_type_indication {
            crate::tydi_memory_representation::TypeIndication::Function => {
                evaluate_var(var.clone(), scope.clone(), evaluator.clone())?;
            },
            _ => (),
        }

        let var_value = var.read().unwrap().get_value();
        match var_value {
            crate::tydi_memory_representation::TypedValue::LogicTypeValue(logic_type) => {
                let logic_type = logic_type.read().unwrap();
                match &*logic_type {
                    crate::tydi_memory_representation::LogicType::LogicGroupType(logic_group) => {
                        match logic_group.read().unwrap().get_template_args() {
                            Some(_) => continue,
                            None => (),
                        }
                        let group_scope = logic_group.read().unwrap().get_scope();
                        check_scope(group_scope.clone(), evaluator.clone())?;
                    },
                    crate::tydi_memory_representation::LogicType::LogicUnionType(logic_union) => {
                        match logic_union.read().unwrap().get_template_args() {
                            Some(_) => continue,
                            None => (),
                        }
                        let union_scope = logic_union.read().unwrap().get_scope();
                        check_scope(union_scope.clone(), evaluator.clone())?;
                    },
                    _ => (),
                }
            },
            crate::tydi_memory_representation::TypedValue::Implementation(implementation) => {
                match implementation.read().unwrap().get_template_args() {
                    Some(_) => continue,
                    None => (),
                }
                let implementation_scope = implementation.read().unwrap().get_scope();
                check_scope(implementation_scope.clone(), evaluator.clone())?;
            },
            crate::tydi_memory_representation::TypedValue::Streamlet(streamlet) => {
                match streamlet.read().unwrap().get_template_args() {
                    Some(_) => continue,
                    None => (),
                }
                let streamlet_scope = streamlet.read().unwrap().get_scope();
                check_scope(streamlet_scope.clone(), evaluator.clone())?;
            },
            _ => (), //we can skip these variables
        }
    }

    return Ok(());
}

pub fn check_project(project: Arc<RwLock<Project>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(), TydiLangError> {
    evaluator.write().unwrap().add_trace(EvaluationTrace::new_region_begin(format!("check assertion")));

    let packages = project.read().unwrap().get_packages();
    for (_, package) in &packages {
        let package_scope = package.read().unwrap().get_scope();
        check_scope(package_scope.clone(), evaluator.clone())?;
    }

    evaluator.write().unwrap().add_trace(EvaluationTrace::new_region_begin(format!("check assertion")));
    return Ok(());
}