use std::sync::{Arc, RwLock};

use crate::tydi_parser::*;
use crate::error::TydiLangError;

use crate::tydi_memory_representation::{Scope, TypedValue, CodeLocation};

use super::{Evaluator, evaluate_expression};

pub fn evaluate_IntExp(exp: String) -> Result<i128, TydiLangError> {
    let n_exp = exp.replace("_", "");
    if n_exp.contains("0x") {
        let n_exp = n_exp.replace("0x", "");
        match i128::from_str_radix(&n_exp, 16) {
            Ok(val) => return Ok(val),
            Err(e) => return Err(TydiLangError::new(format!("{} is not a hex integer", exp), CodeLocation::new_unknown())),
        }
    }
    if n_exp.contains("0b") {
        let n_exp = n_exp.replace("0b", "");
        match i128::from_str_radix(&n_exp, 2) {
            Ok(val) => return Ok(val),
            Err(e) => return Err(TydiLangError::new(format!("{} is not a bin integer", exp), CodeLocation::new_unknown())),
        }
    }
    if n_exp.contains("0o") {
        let n_exp = n_exp.replace("0o", "");
        match i128::from_str_radix(&n_exp, 8) {
            Ok(val) => return Ok(val),
            Err(e) => return Err(TydiLangError::new(format!("{} is not an oct integer", exp), CodeLocation::new_unknown())),
        }
    }
    let normal_int_parse_result = n_exp.parse::<i128>();
    if normal_int_parse_result.is_err() {
        return Err(TydiLangError::new(format!("{} is not an integer", exp), CodeLocation::new_unknown()));
    }
    return Ok(normal_int_parse_result.unwrap());
}

pub fn evaluate_StringExp(exp: String) -> Result<String, TydiLangError> {
    let mut n_exp = exp.clone();
    match n_exp.chars().next() {
        Some('\"') => { n_exp = n_exp.chars().skip(1).collect::<String>(); }
        _ => return Err(TydiLangError::new(format!("{} is not a string", exp), CodeLocation::new_unknown()))
    }
    match n_exp.chars().last() {
        Some('\"') => { n_exp.pop(); }
        _ => return Err(TydiLangError::new(format!("{} is not a string", exp), CodeLocation::new_unknown()))
    }
    return Ok(n_exp);
}

pub fn evaluate_BoolExp(exp: String) -> Result<bool, TydiLangError> {
    let n_exp = exp.trim().to_string();
    if n_exp == String::from("true") {
        return Ok(true);
    }
    if n_exp == String::from("false") {
        return Ok(false);
    }
    return Err(TydiLangError::new(format!("{} is not a boolean", exp), CodeLocation::new_unknown()));
}

pub fn evaluate_FloatExp(exp: String) -> Result<f64, TydiLangError> {
    match exp.parse::<f64>() {
        Ok(val) => return Ok(val),
        Err(e) => return Err(TydiLangError::new(format!("{} is not a floating number", exp), CodeLocation::new_unknown())),
    }
}

pub fn evaluate_Term(term: Pair<Rule>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let output_value = TypedValue::UnknwonValue;
    for element in term.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                let exp = element.as_str().to_string();
                let value = evaluate_expression(exp, scope.clone(), evaluator.clone())?;
                return Ok(value);
            }
            Rule::ArrayRange => {
                todo!()
            }
            Rule::ArrayExp => {
                todo!()
            }
            Rule::IndexExp => {
                todo!()
            }
            Rule::IntExp => {
                let int_exp = element.as_str().to_string();
                let value = evaluate_IntExp(int_exp)?;
                return Ok(TypedValue::IntValue(value));
            }
            Rule::StringExp => {
                let string_exp = element.as_str().to_string();
                let value = evaluate_StringExp(string_exp)?;
                return Ok(TypedValue::StringValue(value));
            }
            Rule::BoolExp => {
                let string_exp = element.as_str().to_string();
                let value = evaluate_BoolExp(string_exp)?;
                return Ok(TypedValue::BoolValue(value));
            }
            Rule::FloatExp => {
                let string_exp = element.as_str().to_string();
                let value = evaluate_FloatExp(string_exp)?;
                return Ok(TypedValue::FloatValue(value));
            }
            Rule::IdentifierWithArgExp => {
                todo!()
            }
            Rule::UnaryExp => {
                todo!()
            }
            _ => unreachable!()
        }
    }

    return Ok(output_value);
}