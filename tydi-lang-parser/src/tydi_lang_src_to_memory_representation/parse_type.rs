use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::tydi_memory_representation::{Scope, TypeIndication, Variable, CodeLocation};
use crate::tydi_lang_src_to_memory_representation::{parse_var::*, parse_logic_type::*};
use crate::tydi_parser::*;

pub fn parse_TypeIndicator(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(TypeIndication, bool), TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    let mut is_array = false;
    let mut array_var = None;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::TypeIndicator_All => {            // int, type, 
                (type_indicator, array_var) = parse_TypeIndicator_All(element, scope.clone())?;
                if array_var.is_some() {
                    return Err(TydiLangError {
                        message: format!("unknown syntax"),
                        location: CodeLocation::new_from_pest_rule(&src),
                    });
                }
            }
            Rule::TypeIndicator_Array => {          // [int], [type]
                is_array = true;
                type_indicator = parse_TypeIndicator_Array(element, scope.clone())?;
            }
            _ => todo!()
        }
    }
    return Ok((type_indicator, is_array));
}

pub fn parse_TypeIndicator_Array(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<TypeIndication, TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    let mut array_var = None;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::TypeIndicator_All => {
                (type_indicator, array_var) = parse_TypeIndicator_All(element, scope.clone())?;
                if array_var.is_some() {
                    return Err(TydiLangError {
                        message: format!("unknown syntax"),
                        location: CodeLocation::new_from_pest_rule(&src),
                    });
                }
            }
            _ => todo!()
        }
    }
    return Ok(type_indicator);
}

/// return: ( TypeIndication, a var to indicate the array size of logic type: None = single var, Some= array )
pub fn parse_TypeIndicator_All(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(TypeIndication, Option<Arc<RwLock<Variable>>>), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::BasicTypeKeyword => {
                let type_indicator = parse_BasicTypeKeyword(element, scope.clone())?;
                return Ok((type_indicator, None));
            }
            Rule::LogicalType => {
                let (type_indicator, array_size_var) = parse_LogicalType(element, scope.clone())?;
                return Ok((type_indicator, array_size_var));
            }
            _ => todo!()
        }
    }
    todo!();
}

pub fn parse_BasicTypeKeyword(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<TypeIndication, TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::IntTypeKeyword => {
                type_indicator = TypeIndication::Int;
            }
            Rule::StringTypeKeyword => {
                type_indicator = TypeIndication::String;
            }
            Rule::BoolTypeKeyword => {
                type_indicator = TypeIndication::Bool;
            }
            Rule::FloatTypeKeyword => {
                type_indicator = TypeIndication::Float;
            }
            Rule::LogicalTypeKeyword => {
                type_indicator = TypeIndication::AnyLogicType;
            }
            _ => todo!()
        }
    }
    return Ok(type_indicator);
}

pub fn parse_BasicTypeKeywordArray(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<TypeIndication, TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::BasicTypeKeyword => {
                type_indicator = parse_BasicTypeKeyword(element, scope.clone())?;
            }
            _ => todo!()
        }
    }
    return Ok(type_indicator);
}

pub fn parse_AllTypeKeyword(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(TypeIndication, bool), TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    let mut is_array = false;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::BasicTypeKeywordArray => {
                type_indicator = parse_BasicTypeKeywordArray(element, scope.clone())?;
                is_array = true;
            }
            Rule::BasicTypeKeyword => {
                type_indicator = parse_BasicTypeKeyword(element, scope.clone())?;
            }
            Rule::StreamletKeyword => {
                type_indicator = TypeIndication::AnyStreamlet;
            }
            Rule::ImplementationKeyword => {
                type_indicator = TypeIndication::AnyImplementation;
            }
            _ => todo!()
        }
    }
    return Ok((type_indicator, is_array));
}

///return: (type_indication, option<array_variable>)
pub fn parse_LogicalType(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(TypeIndication, Option<Arc<RwLock<Variable>>>), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::LogicalType_Array => {
                let (type_indicator, array_size_var) = parse_LogicalType_Array(element, scope.clone())?;
                return Ok((type_indicator, array_size_var));
            }
            Rule::LogicalType_Basic => {
                let type_indicator = parse_LogicalType_Basic(element, scope.clone())?;
                return Ok((type_indicator, None));
            }
            _ => todo!()
        }
    }
    todo!()
}

pub fn parse_LogicalType_Basic(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<TypeIndication, TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::LogicalNull => {
                type_indicator = TypeIndication::LogicNull;
            }
            Rule::LogicalBit => {
                let logic_bit_var = parse_LogicalBit(element, scope.clone())?;
                type_indicator = TypeIndication::LogicBit(logic_bit_var);
            }
            Rule::LogicalStream => {
                todo!()
                // type_indicator = TypeIndication::LogicStream;


            }
            Rule::LogicalGroup => {
                

                todo!()
                // type_indicator = TypeIndication::LogicGroup;


            }
            Rule::LogicalUnion => {
                todo!()
                // type_indicator = TypeIndication::LogicUnion;


            }
            _ => todo!()
        }
    }
    return Ok(type_indicator);
}

pub fn parse_LogicalType_Array(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(TypeIndication, Option<Arc<RwLock<Variable>>>), TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    let mut array_size_var_opt = None;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::LogicalType_Basic => {
                type_indicator = parse_LogicalType_Basic(element, scope.clone())?;
            }
            Rule::ArraySizeIndicator => {
                array_size_var_opt = parse_ArraySizeIndicator(element, scope.clone())?;
            }
            _ => todo!()
        }
    }
    return Ok((type_indicator, array_size_var_opt));
}

pub fn parse_ArraySizeIndicator(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Option<Arc<RwLock<Variable>>>, TydiLangError> {
    let mut is_exp_provided = false;
    let mut array_size_var = Variable::new_place_holder();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                is_exp_provided = true;
                array_size_var = create_variable_from_exp(element, scope.clone())?;
            }
            _ => todo!()
        }
    }
    if is_exp_provided {
        {
            let mut array_size_var_write = array_size_var.write().unwrap();
            array_size_var_write.set_type_indication(TypeIndication::Int);
        }
        return Ok(Some(array_size_var));
    }
    else {
        return Ok(None);
    }
}
