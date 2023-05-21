use std::sync::{Arc, RwLock};

use crate::{tydi_memory_representation::{TypedValue, CodeLocation, Scope, ScopeRelationType, GetScope}};
use crate::{error::TydiLangError, trait_common::GetName};

use super::{Expression, Operator, Evaluator, evaluate_var, evaluate_id_in_typed_value};


pub fn evaluate_BinaryOperation(lhs: &Box<Expression>, op: &Operator, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    match op {
        Operator::Unknown => unreachable!(),
        Operator::AccessInner => {
            return perform_AccessInner(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::AccessProperty => todo!(),
        Operator::LeftShift => {
            return perform_LeftShift(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::RightShift => {
            return perform_RightShift(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::LogicalAnd => {
            return perform_LogicalAnd(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::LogicalOr => {
            return perform_LogicalOr(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::LogicalEq => {
            return perform_LogicalEq(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::LogicalNotEq => {
            return perform_LogicalNotEq(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::GreaterEq => {
            return perform_GreaterEq(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::LessEq => {
            return perform_LessEq(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::Greater => {
            return perform_Greater(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::Less => {
            return perform_Less(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::Add => {
            return perform_Add(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::Minus => {
            return perform_Minus(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::Multiply => {
            return perform_Multiply(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::Divide => {
            return perform_Divide(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::Mod => {
            return perform_Mod(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::BitAnd => {
            return perform_BitAnd(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::BitOr => {
            return perform_BitOr(lhs, rhs, scope.clone(), evaluator.clone());
        },
        Operator::BitXor => {
            return perform_BitXor(lhs, rhs, scope.clone(), evaluator.clone());
        },
    }

}

//access an identifier in other scopes: e.g. i.x
#[allow(non_snake_case)]
pub fn perform_AccessInner(lhs: &Box<Expression>, rhs: &Box<Expression>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    let lhs_value = evaluate_id_in_typed_value(lhs_value, scope.clone(), evaluator.clone())?;
    let rhs_value = rhs.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
    // let rhs_value = evaluate_id_in_typed_value(rhs_value, scope.clone(), evaluator.clone())?;    //we don't try to evaluate the id of rhs_value since its scope is unknown

    let scope_of_rhs_var = match &lhs_value {
        TypedValue::PackageReferenceValue(package_ref) => {
            let package_scope = package_ref.read().unwrap().get_scope();
            package_scope
        },
        TypedValue::LogicTypeValue(v) => {
            let logic_type = v.read().unwrap();
            let output_scope = match &*logic_type {
                crate::tydi_memory_representation::LogicType::LogicNullType => return Err(TydiLangError::new(format!("LogicNull does not have scope"), CodeLocation::new_unknown())),
                crate::tydi_memory_representation::LogicType::LogicBitType(_) => todo!(),
                crate::tydi_memory_representation::LogicType::LogicGroupType(v) => {
                    v.read().unwrap().get_scope()
                },
                crate::tydi_memory_representation::LogicType::LogicUnionType(v) => {
                    v.read().unwrap().get_scope()
                },
                crate::tydi_memory_representation::LogicType::LogicStreamType(_) => todo!(),
            };
            output_scope
        },
        TypedValue::Streamlet(v) => {
            todo!()
        },
        TypedValue::Implementation(v) => {
            todo!()
        }
        TypedValue::RefToVar(_) => unreachable!(),
        TypedValue::Identifier(_) => unreachable!(),
        _ => unreachable!()
    };
    
    //get rhs var name
    let rhs_var_name = match rhs_value {
        TypedValue::Identifier(id) => {
            id.read().unwrap().get_id()
        }
        _ => unreachable!()
    };

    let (rhs_var, rhs_var_scope) = Scope::resolve_identifier(&rhs_var_name, scope_of_rhs_var.clone(), ScopeRelationType::resolve_id_default())?;
    let rhs_typed_value = evaluate_var(rhs_var.clone(), rhs_var_scope.clone(), evaluator.clone())?;
    return Ok(rhs_typed_value);
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
