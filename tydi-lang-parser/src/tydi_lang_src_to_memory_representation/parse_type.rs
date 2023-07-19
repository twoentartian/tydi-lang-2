use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::tydi_memory_representation::{Scope, TypeIndication, Variable, CodeLocation, TraitCodeLocationAccess};
use crate::tydi_lang_src_to_memory_representation::parse_logic_type::*;
use crate::tydi_parser::*;
use crate::generate_name;

#[allow(non_snake_case)]
pub fn parse_TypeIndicator(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<TypeIndication, TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::TypeIndicator_All => {            // int, type, 
                type_indicator = parse_TypeIndicator_All(element, scope.clone(), raw_src.clone())?;
            }
            Rule::TypeIndicator_Array => {          // [int], [type]
                type_indicator = TypeIndication::Array(Box::new(parse_TypeIndicator_Array(element, scope.clone(), raw_src.clone())?));
            }
            _ => unreachable!()
        }
    }
    return Ok(type_indicator);
}

#[allow(non_snake_case)]
pub fn parse_TypeIndicator_Array(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<TypeIndication, TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::TypeIndicator_All => {
                type_indicator = parse_TypeIndicator_All(element, scope.clone(), raw_src.clone())?;
            }
            _ => unreachable!()
        }
    }
    return Ok(type_indicator);
}

#[allow(non_snake_case)]
/// return: ( TypeIndication, a var to indicate the array size of logic type: None = single var, Some= array )
pub fn parse_TypeIndicator_All(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<TypeIndication, TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::BasicTypeKeyword => {
                let type_indicator = parse_BasicTypeKeyword(element, scope.clone(), raw_src.clone())?;
                return Ok(type_indicator);
            }
            Rule::LogicalType => {
                let type_indicator = parse_LogicalType(element, scope.clone(), raw_src.clone())?;
                return Ok(type_indicator);
            }
            Rule::Exp => {
                //the exp means it's a logic type in current scope or other scope
                let exp = element.as_str().to_string();
                return Ok(TypeIndication::LogicTypeRef(exp));
            }
            _ => unreachable!()
        }
    }
    todo!();
}

#[allow(non_snake_case)]
pub fn parse_BasicTypeKeyword(src: Pair<Rule>, _scope: Arc<RwLock<Scope>>, _: Arc<String>) -> Result<TypeIndication, TydiLangError> {
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
            _ => unreachable!()
        }
    }
    return Ok(type_indicator);
}

#[allow(non_snake_case)]
pub fn parse_BasicTypeKeywordArray(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<TypeIndication, TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::BasicTypeKeyword => {
                type_indicator = parse_BasicTypeKeyword(element, scope.clone(), raw_src.clone())?;
            }
            _ => unreachable!()
        }
    }
    return Ok(type_indicator);
}

#[allow(non_snake_case)]
pub fn parse_AllTypeKeyword(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<(TypeIndication, bool), TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    let mut is_array = false;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::BasicTypeKeywordArray => {
                type_indicator = parse_BasicTypeKeywordArray(element, scope.clone(), raw_src.clone())?;
                is_array = true;
            }
            Rule::BasicTypeKeyword => {
                type_indicator = parse_BasicTypeKeyword(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StreamletKeyword => {
                type_indicator = TypeIndication::AnyStreamlet;
            }
            Rule::ImplementationKeyword => {
                type_indicator = TypeIndication::AnyImplementation;
            }
            _ => unreachable!()
        }
    }
    return Ok((type_indicator, is_array));
}

#[allow(non_snake_case)]
///return: (type_indication, option<array_variable>)
pub fn parse_LogicalType(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<TypeIndication, TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::LogicalType_Array => {
                let type_indicator = parse_LogicalType_Array(element, scope.clone(), raw_src.clone())?;
                return Ok(type_indicator);
            }
            Rule::LogicalType_Basic => {
                let type_indicator = parse_LogicalType_Basic(element, scope.clone(), raw_src.clone())?;
                return Ok(type_indicator);
            }
            _ => unreachable!()
        }
    }
    unreachable!()
}

#[allow(non_snake_case)]
pub fn parse_LogicalType_Basic(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<TypeIndication, TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::LogicalNull => {
                type_indicator = TypeIndication::LogicNull;
            }
            Rule::LogicalBit => {
                let logic_bit_var = parse_LogicalBit(element, scope.clone(), raw_src.clone())?;
                type_indicator = TypeIndication::LogicBit(logic_bit_var);
            }
            Rule::LogicalStream => {
                let logic_stream_var = parse_LogicalStream(element, scope.clone(), raw_src.clone())?;
                type_indicator = TypeIndication::LogicStream(logic_stream_var);
            }
            Rule::LogicalGroup => {
                return Err(TydiLangError::new(format!("syntax not allowed, declare group as {{x : Group y {{...}} }} or {{x = Group y {{...}} }}"), CodeLocation::new_from_pest_rule(&src, raw_src.clone())));
            }
            Rule::LogicalUnion => {
                return Err(TydiLangError::new(format!("syntax not allowed, declare union as {{x : Union y {{...}} }} or {{x = Union y {{...}} }}"), CodeLocation::new_from_pest_rule(&src, raw_src.clone())));
            }
            _ => unreachable!()
        }
    }
    return Ok(type_indicator);
}

#[allow(non_snake_case)]
pub fn parse_LogicalType_Array(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<TypeIndication, TydiLangError> {
    let mut type_indicator = TypeIndication::Any;
    let mut array_size_var_opt = None;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::LogicalType_Basic => {
                type_indicator = parse_LogicalType_Basic(element, scope.clone(), raw_src.clone())?;
            }
            Rule::ArraySizeIndicator => {
                array_size_var_opt = parse_ArraySizeIndicator(element, scope.clone(), raw_src.clone())?;
            }
            _ => unreachable!()
        }
    }

    if array_size_var_opt.is_some() {
        let type_indicator_var = match &type_indicator {
            TypeIndication::LogicNull => return Err(TydiLangError::new(format!("Logic Null cannot be an arry"), CodeLocation::new_from_pest_rule(&src, raw_src.clone()))),
            TypeIndication::LogicStream(_) => return Err(TydiLangError::new(format!("Logic Stream cannot be an arry"), CodeLocation::new_from_pest_rule(&src, raw_src.clone()))),
            TypeIndication::LogicBit(v) => v,
            TypeIndication::LogicGroup(v) => v,
            TypeIndication::LogicUnion(v) => v,
            _ => unreachable!()
        };
        {
            let mut type_indicator_var_write = type_indicator_var.write().unwrap();
            type_indicator_var_write.set_array_size(array_size_var_opt);
        }
    }   //we need to set the value according to the "array_size" during evaluation

    return Ok(type_indicator);
}

#[allow(non_snake_case)]
pub fn parse_ArraySizeIndicator(src: Pair<Rule>, _scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Option<Arc<RwLock<Variable>>>, TydiLangError> {
    let mut is_exp_provided = false;
    let mut array_size_var = Variable::new_place_holder();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                is_exp_provided = true;
                // array_size_var = create_variable_from_exp(element, scope.clone())?;
                let name = generate_name::generate_built_in_variable_name_from_span(&element);
                array_size_var = Variable::new_with_type_indication(name, Some(element.as_str().to_string()), TypeIndication::Int);
                {
                    let mut array_size_var_write = array_size_var.write().unwrap();
                    array_size_var_write.set_code_location(CodeLocation::new_from_pest_rule(&element, raw_src.clone()));
                }
            }
            _ => unreachable!()
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
