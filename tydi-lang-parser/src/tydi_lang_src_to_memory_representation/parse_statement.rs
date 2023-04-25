use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::trait_common::GetName;
use crate::tydi_memory_representation::{Scope, Variable, TraitCodeLocationAccess, CodeLocation, TypeIndication};
use crate::tydi_parser::*;
use crate::tydi_lang_src_to_memory_representation::parse_type::{parse_TypeIndicator};

use super::parse_logic_type::parse_LogicalGroup;

pub fn parse_StatementDeclareVariable(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), TydiLangError> {
    let var = Variable::new_place_holder();
    let mut type_indicator = TypeIndication::Any;
    let mut is_array = false;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                var.write().unwrap().set_name(element.as_str().to_string());
            }
            Rule::TypeIndicator => {
                (type_indicator, is_array) = parse_TypeIndicator(element, scope.clone())?;
            }
            Rule::Exp => {
                var.write().unwrap().set_exp(Some(element.as_str().to_string()));
            }
            _ => todo!()
        }
    }
    {
        let mut var_write = var.write().unwrap();
        var_write.set_type_indication(type_indicator.clone());
        var_write.set_is_array(is_array);
        let loc = CodeLocation::new_from_pest_rule(&src);
        var_write.set_code_location(loc);
    }
    {
        let mut scope_write = scope.write().unwrap();
        scope_write.add_var(var)?;
    }
    return Ok(());
}

pub fn parse_StatementDeclareType(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), TydiLangError> {
    let var = Variable::new_place_holder();
    let mut type_indicator = TypeIndication::Any;
    let mut is_array = false;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                var.write().unwrap().set_name(element.as_str().to_string());
            }
            Rule::TypeIndicator => {
                (type_indicator, is_array) = parse_TypeIndicator(element, scope.clone())?;
            }
            _ => todo!()
        }
    }
    
    {
        let mut var_write = var.write().unwrap();
        var_write.set_type_indication(type_indicator.clone());
        var_write.set_is_property_of_scope(true);
        var_write.set_is_array(is_array);
        let loc = CodeLocation::new_from_pest_rule(&src);
        var_write.set_code_location(loc);

        //if it is a logic type (excluding LogicNull):
        match &type_indicator {
            TypeIndication::LogicStream(v) | 
            TypeIndication::LogicBit(v) | 
            TypeIndication::LogicGroup(v) |
            TypeIndication::LogicUnion(v) => {
                var_write.set_exp(Some(v.read().unwrap().get_name()));
                var_write.set_type_indication(TypeIndication::AnyLogicType);
            },
            _ => ()
        }
    }
    {
        let mut scope_write = scope.write().unwrap();
        scope_write.add_var(var)?;
    }

    return Ok(());
}

pub fn parse_StatementDeclareGroup(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), TydiLangError> {
    let mut var = Variable::new_place_holder();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::LogicalGroup => {
                var = parse_LogicalGroup(element, scope.clone())?;
            }
            _ => todo!()
        }
    }
    {
        let mut scope_write = scope.write().unwrap();
        scope_write.add_var(var)?;
    }

    return Ok(());
}