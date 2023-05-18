use crate::{tydi_memory_representation::{TypedValue, CodeLocation}, error::TydiLangError};

use super::{Expression, Operator};


pub fn evaluate_BinaryOperation(lhs: &Box<Expression>, op: &Operator, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    match op {
        Operator::Unknown => unreachable!(),
        Operator::AccessInner => todo!(),
        Operator::AccessProperty => todo!(),
        Operator::LeftShift => {
            return perform_LeftShift(lhs, rhs);
        },
        Operator::RightShift => {
            return perform_RightShift(lhs, rhs);
        },
        Operator::LogicalAnd => {
            return perform_LogicalAnd(lhs, rhs);
        },
        Operator::LogicalOr => {
            return perform_LogicalOr(lhs, rhs);
        },
        Operator::LogicalEq => {
            return perform_LogicalEq(lhs, rhs);
        },
        Operator::LogicalNotEq => {
            return perform_LogicalNotEq(lhs, rhs);
        },
        Operator::GreaterEq => {
            return perform_GreaterEq(lhs, rhs);
        },
        Operator::LessEq => {
            return perform_LessEq(lhs, rhs);
        },
        Operator::Greater => {
            return perform_Greater(lhs, rhs);
        },
        Operator::Less => {
            return perform_Less(lhs, rhs);
        },
        Operator::Add => {
            return perform_Add(lhs, rhs);
        },
        Operator::Minus => {
            return perform_Minus(lhs, rhs);
        },
        Operator::Multiply => {
            return perform_Multiply(lhs, rhs);
        },
        Operator::Divide => {
            return perform_Divide(lhs, rhs);
        },
        Operator::Mod => {
            return perform_Mod(lhs, rhs);
        },
        Operator::BitAnd => {
            return perform_BitAnd(lhs, rhs);
        },
        Operator::BitOr => {
            return perform_BitOr(lhs, rhs);
        },
        Operator::BitXor => {
            return perform_BitXor(lhs, rhs);
        },
    }

}

#[allow(non_snake_case)]
pub fn perform_Add(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
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
pub fn perform_Minus(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
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

#[allow(non_snake_case)]
pub fn perform_Multiply(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
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

#[allow(non_snake_case)]
pub fn perform_Divide(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
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

#[allow(non_snake_case)]
pub fn perform_Mod(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
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



#[allow(non_snake_case)]
pub fn perform_BitAnd(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::IntValue(v0&v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("BitAnd not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_BitOr(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::IntValue(v0|v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("BitOr not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_BitXor(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::IntValue(v0^v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("BitXor not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}


#[allow(non_snake_case)]
pub fn perform_LeftShift(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::IntValue(v0 << v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("LeftShift not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_RightShift(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
    match (lhs_value, rhs_value) {
        (TypedValue::IntValue(v0), TypedValue::IntValue(v1)) => {
            return Ok(TypedValue::IntValue(v0 >> v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("RightShift not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_LogicalAnd(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
    match (lhs_value, rhs_value) {
        (TypedValue::BoolValue(v0), TypedValue::BoolValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 && v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("LogicalAnd not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_LogicalOr(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
    match (lhs_value, rhs_value) {
        (TypedValue::BoolValue(v0), TypedValue::BoolValue(v1)) => {
            return Ok(TypedValue::BoolValue(v0 || v1));
        },
        (v0, v1) => return Err(TydiLangError::new(format!("LogicalOr not supported for {:?} and {:?}", v0 ,v1), CodeLocation::new_unknown()))
    }
}

#[allow(non_snake_case)]
pub fn perform_LogicalEq(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
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
pub fn perform_LogicalNotEq(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let value = perform_LogicalEq(lhs,rhs)?;
    let output = match value {
        TypedValue::BoolValue(v) => TypedValue::BoolValue(!v),
        _ => unreachable!()
    };
    return Ok(output);
}

#[allow(non_snake_case)]
pub fn perform_Greater(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
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
pub fn perform_Less(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
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
pub fn perform_GreaterEq(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
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
pub fn perform_LessEq(lhs: &Box<Expression>, rhs: &Box<Expression>) -> Result<TypedValue, TydiLangError> {
    let lhs_value = lhs.evaluate_TypedValue()?;
    let rhs_value = rhs.evaluate_TypedValue()?;
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
