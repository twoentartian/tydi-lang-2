use crate::{tydi_memory_representation::{TypedValue, CodeLocation}, error::TydiLangError};

use super::{Expression, Operator};


pub fn evaluate_BinaryOperation(lhs: &Box<Expression>, op: &Operator, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    match op {
        Operator::AccessInner => todo!(),
        Operator::AccessProperty => todo!(),
        Operator::LeftShift => todo!(),
        Operator::RightShift => todo!(),
        Operator::LogicalAnd => todo!(),
        Operator::LogicalOr => todo!(),
        Operator::LogicalEq => todo!(),
        Operator::LogicalNotEq => todo!(),
        Operator::GreaterEq => todo!(),
        Operator::LessEq => todo!(),
        Operator::Greater => todo!(),
        Operator::Less => todo!(),
        Operator::Add => {
            return perform_add(lhs, rhs);
        },
        Operator::Minus => {
            return perform_minus(lhs, rhs);
        },
        Operator::Multiply => {
            return perform_multiply(lhs, rhs);
        },
        Operator::Divide => {
            return perform_divide(lhs, rhs);
        },
        Operator::Mod => {
            return perform_mod(lhs, rhs);
        },
        Operator::BitAnd => todo!(),
        Operator::BitOr => todo!(),
        Operator::BitXor => todo!(),
    }

}

pub fn perform_add(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::IntValue(v0+v1)),
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0+v1)),

        (TypedValue::IntValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0 as f64 + v1)),
        (TypedValue::FloatValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::FloatValue(v0 + v1 as f64)),

        (TypedValue::StringValue(v0), TypedValue::StringValue(v1)) => return Ok(TypedValue::StringValue(v0 + &v1)),

        (v0, v1) => return Err(TydiLangError::new(format!("addition not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

pub fn perform_minus(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::IntValue(v0-v1)),
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0-v1)),

        (TypedValue::IntValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0 as f64 - v1)),
        (TypedValue::FloatValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::FloatValue(v0 - v1 as f64)),

        (v0, v1) => return Err(TydiLangError::new(format!("addition not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

pub fn perform_multiply(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::IntValue(v0*v1)),
        (TypedValue::FloatValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0*v1)),

        (TypedValue::IntValue(v0), TypedValue::FloatValue(v1)) => return Ok(TypedValue::FloatValue(v0 as f64 * v1)),
        (TypedValue::FloatValue(v0), TypedValue::IntValue(v1)) => return Ok(TypedValue::FloatValue(v0 * v1 as f64)),

        (v0, v1) => return Err(TydiLangError::new(format!("addition not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

pub fn perform_divide(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
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

pub fn perform_mod(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
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