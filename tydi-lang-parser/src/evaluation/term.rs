use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use crate::generate_name::generate_init_value;
use crate::tydi_parser::*;
use crate::error::TydiLangError;

use crate::tydi_memory_representation::{Scope, TypedValue, CodeLocation, Identifier, IdentifierType, ScopeRelationType};

use super::{Evaluator, evaluate_expression_pest, evaluate_id_in_typed_value, UnaryOperator};

#[allow(non_snake_case)]
pub fn evaluate_Term(term: Pair<Rule>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let output_value = TypedValue::UnknwonValue;
    for element in term.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                let exp = evaluate_expression_pest(element, None, scope.clone(), evaluator.clone())?;
                let exp_typed_value = exp.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
                return Ok(exp_typed_value);
            }
            Rule::ArrayExp => {
                let value = evaluate_ArrayExp(element, scope.clone(), evaluator.clone())?;
                return Ok(value);
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
                //we cannot directly do evaluation here because a term is only a single id, access / access inner will not work here
                let value = parse_IdentifierWithArgExp(element, scope.clone(), evaluator.clone())?;
                return Ok(value);
            }
            Rule::UnaryExp => {
                let value = evaluate_UnaryExp(element, scope.clone(), evaluator.clone())?;
                return Ok(value);
            }
            _ => unreachable!()
        }
    }

    return Ok(output_value);
}


#[allow(non_snake_case)]
pub fn evaluate_IntExp(exp: String) -> Result<i128, TydiLangError> {
    let n_exp = exp.trim().to_string();
    let n_exp = n_exp.replace("_", "");
    if n_exp.contains("0x") {
        let n_exp = n_exp.replace("0x", "");
        match i128::from_str_radix(&n_exp, 16) {
            Ok(val) => return Ok(val),
            Err(_) => return Err(TydiLangError::new(format!("{} is not a hex integer", exp), CodeLocation::new_unknown())),
        }
    }
    if n_exp.contains("0b") {
        let n_exp = n_exp.replace("0b", "");
        match i128::from_str_radix(&n_exp, 2) {
            Ok(val) => return Ok(val),
            Err(_) => return Err(TydiLangError::new(format!("{} is not a bin integer", exp), CodeLocation::new_unknown())),
        }
    }
    if n_exp.contains("0o") {
        let n_exp = n_exp.replace("0o", "");
        match i128::from_str_radix(&n_exp, 8) {
            Ok(val) => return Ok(val),
            Err(_) => return Err(TydiLangError::new(format!("{} is not an oct integer", exp), CodeLocation::new_unknown())),
        }
    }
    let normal_int_parse_result = n_exp.parse::<i128>();
    if normal_int_parse_result.is_err() {
        return Err(TydiLangError::new(format!("{} is not an integer", exp), CodeLocation::new_unknown()));
    }
    return Ok(normal_int_parse_result.unwrap());
}

#[allow(non_snake_case)]
pub fn evaluate_StringExp(exp: String) -> Result<String, TydiLangError> {
    let mut n_exp = exp.clone();
    n_exp = n_exp.trim().to_string();
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

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub fn evaluate_FloatExp(exp: String) -> Result<f64, TydiLangError> {
    let n_exp = exp.trim().to_string();
    match n_exp.parse::<f64>() {
        Ok(val) => return Ok(val),
        Err(_) => return Err(TydiLangError::new(format!("{} is not a floating number", exp), CodeLocation::new_unknown())),
    }
}

#[allow(non_snake_case)]
pub fn evaluate_ArrayExp(exps: Pair<Rule>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let mut output = vec![];
    for element in exps.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                let element_exp = evaluate_expression_pest(element, None, scope.clone(), evaluator.clone())?;
                let element_typed_value = element_exp.evaluate_TypedValue(scope.clone(), evaluator.clone())?;
                let element_typed_value = evaluate_id_in_typed_value(element_typed_value, None, ScopeRelationType::resolve_id_default(), scope.clone(), evaluator.clone())?;
                output.push(element_typed_value);
            }
            _ => unreachable!()
        }
    }
    return Ok(TypedValue::Array(output));
}

#[allow(non_snake_case)]
pub fn evaluate_UnaryExp(exp: Pair<Rule>, scope: Arc<RwLock<Scope>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let mut exp_typed_value = TypedValue::UnknwonValue;
    let mut unary_operator = UnaryOperator::Unknown;
    for element in exp.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Term => {
                exp_typed_value = evaluate_Term(element, scope.clone(), evaluator.clone())?;
            }
            Rule::OP_UnaryMinus => {
                unary_operator = UnaryOperator::OP_UnaryMinus;
            }
            Rule::OP_UnaryNot => {
                unary_operator = UnaryOperator::OP_UnaryNot;
            }
            _ => unreachable!()
        }
    }
    
    #[allow(unused_assignments)]
    let mut output_typed_value = TypedValue::UnknwonValue;
    match unary_operator {
        UnaryOperator::OP_UnaryMinus => {
            match exp_typed_value {
                TypedValue::IntValue(v) => {
                    output_typed_value = TypedValue::IntValue(-v);
                },
                TypedValue::FloatValue(v) => {
                    output_typed_value = TypedValue::FloatValue(-v);
                },
                _ => {
                    return Err(TydiLangError::new(format!("unary not operator {:?} cannot be applied to {:?}", unary_operator, exp_typed_value), CodeLocation::new_unknown()));
                }
            }
        },
        UnaryOperator::OP_UnaryNot => {
            match exp_typed_value {
                TypedValue::BoolValue(v) => {
                    output_typed_value = TypedValue::BoolValue(!v);
                },
                _ => {
                    return Err(TydiLangError::new(format!("unary not operator {:?} cannot be applied to {:?}", unary_operator, exp_typed_value), CodeLocation::new_unknown()));
                }
            }
        },
        _ => {
            todo!("unknown unary operator: {:?}", unary_operator);
        }
    }

    return Ok(output_typed_value);
}

#[allow(non_snake_case)]
pub fn parse_IdentifierWithArgExp(id: Pair<Rule>, _scope: Arc<RwLock<Scope>>, _evaluator: Arc<RwLock<Evaluator>>) -> Result<TypedValue, TydiLangError> {
    let mut id_type = IdentifierType::Unknown;
    let mut id_name = generate_init_value();
    let mut template_exps = BTreeMap::new();
    for element in id.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Term_identifier => {
                (id_name, id_type) = parse_Term_identifier(element)?;
            }
            Rule::Exp => {
                template_exps.insert(template_exps.len(), element.as_str().to_string());
            }
            _ => unreachable!()
        }
    }
    let output_id = Identifier::new(id_name, id_type, template_exps, CodeLocation::new_unknown());
    let output_typed_value = TypedValue::Identifier(output_id);
    return Ok(output_typed_value);
}

#[allow(non_snake_case)]
pub fn parse_Term_identifier(id: Pair<Rule>) -> Result<(String, IdentifierType), TydiLangError> {
    let mut id_type = IdentifierType::Unknown;
    let mut id_name = generate_init_value();
    for element in id.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::FunctionExp => {
                (id_name, id_type) = parse_FunctionExp(element)?;
            }
            Rule::IndexExp => {
                (id_name, id_type) = parse_IndexExp(element)?;
            }
            Rule::IdentifierExp => {
                (id_name, id_type) = parse_IdentifierExp(element)?;
            }
            _ => unreachable!()
        }
    }
    return Ok((id_name, id_type));
}

#[allow(non_snake_case)]
pub fn parse_FunctionExp(id: Pair<Rule>) -> Result<(String, IdentifierType), TydiLangError> {
    let mut id_name = generate_init_value();
    let mut function_exps = BTreeMap::new();
    for element in id.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                id_name = element.as_str().to_string();
            }
            Rule::Exp => {
                function_exps.insert(function_exps.len(), element.as_str().to_string());
            }
            _ => unreachable!()
        }
    }
    return Ok((id_name, IdentifierType::FunctionExp(function_exps)));
}

#[allow(non_snake_case)]
pub fn parse_IndexExp(id: Pair<Rule>) -> Result<(String, IdentifierType), TydiLangError> {
    let mut id_name = generate_init_value();
    let mut index_exp = generate_init_value();
    for element in id.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                id_name = element.as_str().to_string();
            }
            Rule::Exp => {
                index_exp = element.as_str().to_string();
            }
            _ => unreachable!()
        }
    }

    return Ok((id_name, IdentifierType::IndexExp(index_exp)));
}

#[allow(non_snake_case)]
pub fn parse_IdentifierExp(id: Pair<Rule>) -> Result<(String, IdentifierType), TydiLangError> {
    let mut id_exp = generate_init_value();
    for element in id.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                id_exp = element.as_str().to_string();
            }
            _ => unreachable!()
        }
    }

    return Ok((id_exp, IdentifierType::IdentifierExp));
}
