use std::{sync::{Arc, RwLock}, clone};

use crate::{tydi_memory_representation::{TypedValue, CodeLocation, Scope, ScopeRelationType, GetScope, streamlet, EvaluationStatus, Variable}, trait_common::AccessProperty};
use crate::{error::TydiLangError, trait_common::GetName};

use super::{Expression, Operator, Evaluator, evaluate_var, evaluate_id_in_typed_value, evaluate_value_with_identifier_type, evaluate_template_exps_of_var};


pub fn evaluate_BinaryOperation(lhs: &Box<Expression>, op: &Operator, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(TypedValue, Option<Arc<RwLock<Variable>>>), TydiLangError> {
    match op {
        Operator::Unknown => unreachable!(),
        Operator::AccessInner => {
            let (value, ref_var) = perform_AccessInner(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, ref_var));
        },
        Operator::AccessProperty => todo!(),
        Operator::LeftShift => {
            let value = perform_LeftShift(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::RightShift => {
            let value = perform_RightShift(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::LogicalAnd => {
            let value = perform_LogicalAnd(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::LogicalOr => {
            let value = perform_LogicalOr(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::LogicalEq => {
            let value = perform_LogicalEq(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::LogicalNotEq => {
            let value = perform_LogicalNotEq(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::GreaterEq => {
            let value = perform_GreaterEq(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::LessEq => {
            let value = perform_LessEq(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::Greater => {
            let value = perform_Greater(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::Less => {
            let value = perform_Less(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::Add => {
            let value = perform_Add(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::Minus => {
            let value = perform_Minus(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::Multiply => {
            let value = perform_Multiply(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::Divide => {
            let value = perform_Divide(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::Mod => {
            let value = perform_Mod(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::BitAnd => {
            let value = perform_BitAnd(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::BitOr => {
            let value = perform_BitOr(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
        Operator::BitXor => {
            let value = perform_BitXor(lhs, rhs, scope.clone(), evaluator.clone())?;
            return Ok((value, None));
        },
    }

}

//access an identifier in other scopes: e.g. i.x
#[allow(non_snake_case)]
pub fn perform_AccessInner(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(TypedValue, Option<Arc<RwLock<Variable>>>), TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let mut lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    //get rhs var name
    let rhs_var_id = match rhs_value {
        TypedValue::Identifier(iden) => {
            iden
        }
        _ => unreachable!()
    };
    let rhs_var_name = rhs_var_id.read().unwrap().get_id();
    let rhs_template_args = rhs_var_id.read().unwrap().get_template_args();
    
    // let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;    //we don't try to evaluate the id of rhs_value since its scope is unknown

    //if the lhs value is a reference
    if let TypedValue::RefToVar(inner_var) = lhs_value.clone() {
        assert!(inner_var.read().unwrap().get_evaluated().is_value_known());
        lhs_value = inner_var.read().unwrap().get_value();
    }

    let (scope_of_rhs_var, resolve_var_scope_edge) = match &lhs_value {
        TypedValue::PackageReferenceValue(package_ref) => {
            let package_scope = package_ref.read().unwrap().get_scope();
            (package_scope, ScopeRelationType::resolve_id_default())
        },
        TypedValue::LogicTypeValue(v) => {
            let logic_type = v.read().unwrap();
            let output_scope = match &*logic_type {
                crate::tydi_memory_representation::LogicType::LogicNullType => return Err(TydiLangError::new(format!("LogicNull does not have scope"), CodeLocation::new_unknown())),
                crate::tydi_memory_representation::LogicType::LogicBitType(bit) => {
                    let var = bit.read().unwrap().access_porperty(&rhs_var_name);
                    if var.is_none() {
                        return Err(TydiLangError::new(format!("LogicBit doesn't have property {}, available: {:?}", &rhs_var_name, crate::tydi_memory_representation::logic_bit::AVAILABLE_PROPERTIES), CodeLocation::new_unknown()));
                    }
                    let var = var.unwrap();

                    assert!(var.read().unwrap().get_evaluated().is_value_known());
                    return Ok((var.read().unwrap().get_value(), Some(var.clone())));
                },
                crate::tydi_memory_representation::LogicType::LogicGroupType(v) => {
                    v.read().unwrap().get_scope()
                },
                crate::tydi_memory_representation::LogicType::LogicUnionType(v) => {
                    v.read().unwrap().get_scope()
                },
                crate::tydi_memory_representation::LogicType::LogicStreamType(stream) => {
                    let var = stream.read().unwrap().access_porperty(&rhs_var_name);
                    if var.is_none() {
                        return Err(TydiLangError::new(format!("LogicStream doesn't have property {}, available: {:?}", &rhs_var_name, crate::tydi_memory_representation::logic_stream::AVAILABLE_PROPERTIES), CodeLocation::new_unknown()));
                    }
                    let var = var.unwrap();
                    assert!(var.read().unwrap().get_evaluated().is_value_known());
                    return Ok((var.read().unwrap().get_value(), Some(var.clone())));
                },
            };
            (output_scope, ScopeRelationType::resolve_id_default())
        },
        TypedValue::Streamlet(v) => {
            let streamlet_scope = v.read().unwrap().get_scope();
            (streamlet_scope, ScopeRelationType::resolve_id_default())
        },
        TypedValue::Implementation(v) => {
            todo!()
        },
        TypedValue::Instance(inst) => {
            let derived_impl_var = inst.read().unwrap().get_derived_impl_var();
            evaluate_var(derived_impl_var.clone(), scope.clone(), evaluator.clone())?;
            let derived_impl_var_value = derived_impl_var.read().unwrap().get_value();
            match derived_impl_var_value {
                TypedValue::Implementation(derived_impl) => {
                    let derived_impl_scope = derived_impl.read().unwrap().get_scope();
                    (derived_impl_scope, ScopeRelationType::resolve_id_in_parent_streamlet())
                }
                _ => unreachable!("we should get an implementation here")
            }
        },
        TypedValue::RefToVar(_) => unreachable!(),
        TypedValue::Identifier(_) => unreachable!(),
        _ => unreachable!()
    };

    let template_exps = evaluate_template_exps_of_var(&rhs_template_args, scope.clone(), evaluator.clone())?;
    let (rhs_var, rhs_var_scope) = Scope::resolve_identifier(&rhs_var_name, &template_exps, scope_of_rhs_var.clone(), resolve_var_scope_edge)?;
    let rhs_typed_value = evaluate_var(rhs_var.clone(), rhs_var_scope.clone(), evaluator.clone())?;
    //if it is an index expression (an element of an array)
    let iden_type = rhs_var_id.read().unwrap().get_id_type();
    let rhs_typed_value = evaluate_value_with_identifier_type(&rhs_var_name, rhs_typed_value, iden_type.clone(), scope.clone(), evaluator.clone())?;
    match iden_type {
        crate::tydi_memory_representation::IdentifierType::FunctionExp(_) => todo!(),
        crate::tydi_memory_representation::IdentifierType::IndexExp(_) => {
            return Ok((rhs_typed_value, None));
        },
        crate::tydi_memory_representation::IdentifierType::IdentifierExp => {
            return Ok((rhs_typed_value, Some(rhs_var.clone())));
        },
        crate::tydi_memory_representation::IdentifierType::Unknown => unreachable!(),
    }
}

#[allow(non_snake_case)]
pub fn perform_Add(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::IntValue(v0+v1)),
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0+v1)),

        (TypedValue::IntValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0 as f64 + v1)),
        (TypedValue::FloatValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::FloatValue(v0 + v1 as f64)),

        (TypedValue::StringValue(v0), TypedValue::StringValue(v1)) => return Ok(TypedValue::StringValue(v0 + &v1)),

        (TypedValue::Array(v0), TypedValue::Array(v1)) => {
            let mut output = vec![];
            for ele in v0 { output.push(ele); }
            for ele in v1 { output.push(ele); }
            return Ok(TypedValue::Array(output));
        },

        (TypedValue::Array(v0), v1) => {
            let mut output = v0;
            output.push(v1);
            return Ok(TypedValue::Array(output));
        },

        (v0, TypedValue::Array(v1)) => {
            let mut output = vec![];
            output.push(v0);
            for ele in v1 { output.push(ele); }
            return Ok(TypedValue::Array(output));
        },

        (v0, v1) => return Err(TydiLangError::new(format!("addition not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_Minus(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::IntValue(v0-v1)),
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0-v1)),

        (TypedValue::IntValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0 as f64 - v1)),
        (TypedValue::FloatValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::FloatValue(v0 - v1 as f64)),

        (v0, v1) => return Err(TydiLangError::new(format!("addition not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_Multiply(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::IntValue(v0*v1)),
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0*v1)),

        (TypedValue::IntValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0 as f64 * v1)),
        (TypedValue::FloatValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::FloatValue(v0 * v1 as f64)),

        (v0, v1) => return Err(TydiLangError::new(format!("multiplication not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_Divide(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            if v1 == 0 { return Err(TydiLangError::new(format!("divide by zero: {}/{}", v0 ,v1), CodeLocation::new_unknown())) }
            return Ok(TypedValue::IntValue(v0/v1));
        },
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => {
            if v1 == 0.0 { return Err(TydiLangError::new(format!("divide by zero: {}/{}", v0 ,v1), CodeLocation::new_unknown())) }
            return Ok(TypedValue::FloatValue(v0/v1));
        },

        (TypedValue::IntValue(v0), TypedValue::FloatValue(v1)) => {
            if v1 == 0.0 { return Err(TydiLangError::new(format!("divide by zero: {}/{}", v0 ,v1), CodeLocation::new_unknown())) }
            return Ok(TypedValue::FloatValue(v0 as f64 / v1));
        },
        (TypedValue::FloatValue(v0), TypedValue::IntValue(v1)) => {
            if v1 == 0 { return Err(TydiLangError::new(format!("divide by zero: {}/{}", v0 ,v1), CodeLocation::new_unknown())) }
            return Ok(TypedValue::FloatValue(v0 / v1 as f64));
        },

        (v0, v1) => return Err(TydiLangError::new(format!("addition not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_Mod(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            if v1 == 0 {
                return Err(TydiLangError::new(format!("modulo by zero: {}%{}", v0 ,v1), CodeLocation::new_unknown()))
            }
            return Ok(TypedValue::IntValue(v0%v1));
        },

        (v0, v1) => return Err(TydiLangError::new(format!("addition not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}



#[allow(non_snake_case)]
pub fn perform_BitAnd(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::IntValue(v0&v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("BitAnd not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_BitOr(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::IntValue(v0|v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("BitOr not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_BitXor(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::IntValue(v0^v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("BitXor not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}


#[allow(non_snake_case)]
pub fn perform_LeftShift(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::IntValue(v0 << v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("LeftShift not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_RightShift(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::IntValue(v0 >> v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("RightShift not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_LogicalAnd(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::BoolValue(v0), TypedValue::BoolValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 && v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("LogicalAnd not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_LogicalOr(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::BoolValue(v0), TypedValue::BoolValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 || v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("LogicalOr not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_LogicalEq(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::BoolValue(v0), TypedValue::BoolValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 == v1));
        },
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 == v1));
        },
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 == v1));
        },
        (TypedValue::StringValue(v0), TypedValue::StringValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 == v1));
        },
        (TypedValue::ClockDomainValue(v0), TypedValue::ClockDomainValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 == v1));
        },
        (TypedValue::LogicTypeValue(v0), TypedValue::LogicTypeValue(v1)) => {
            let logic_type0 = v0.read().unwrap();
            let logic_type1 = v1.read().unwrap();
            return Ok(TypedValue::BoolValue(*logic_type0 == *logic_type1));
        },
        (TypedValue::Array(v0), TypedValue::Array(v1)) => {
            if v0.len() != v1.len() {
                return Ok(TypedValue::BoolValue(false));
            }
            for i in 0..v0.len() {
                let v0_ele = &v0[i];
                let v1_ele = &v1[i];
                if v0_ele != v1_ele {
                    return Ok(TypedValue::BoolValue(false));
                }
            }
            return Ok(TypedValue::BoolValue(true));
        },

        (v0, v1) => return Err(TydiLangError::new(format!("LogicalEq not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_LogicalNotEq(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let value = perform_LogicalEq(lhs,rhs, scope.clone(), evaluator.clone())?;
    let output = match value {
        TypedValue::BoolValue(v) => TypedValue::BoolValue(!v),
        _ => unreachable!()
    };
    return Ok(output);
}

#[allow(non_snake_case)]
pub fn perform_Greater(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 > v1));
        },
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 > v1));
        },

        (v0, v1) => return Err(TydiLangError::new(format!("Greater not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_Less(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 < v1));
        },
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 < v1));
        },

        (v0, v1) => return Err(TydiLangError::new(format!("Less not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_GreaterEq(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 >= v1));
        },
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 >= v1));
        },

        (v0, v1) => return Err(TydiLangError::new(format!("GreaterEq not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_LessEq(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;
    match (lhs_value, rhs_value) {
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 <= v1));
        },
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 <= v1));
        },

        (v0, v1) => return Err(TydiLangError::new(format!("LessEq not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}
