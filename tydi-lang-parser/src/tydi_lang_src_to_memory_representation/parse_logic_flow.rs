use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;

use crate::generate_name::{generate_built_in_variable_name_from_span};
use crate::tydi_memory_representation::{Scope, GetScope, Variable, TraitCodeLocationAccess, CodeLocation, TypeIndication, If, Elif, TypedValue, Else, For};
use crate::tydi_parser::*;

use crate::tydi_lang_src_to_memory_representation::{parse_file};

#[allow(non_snake_case)]
pub fn parse_If(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let output_if = If::new(generate_built_in_variable_name_from_span(&src), scope.clone());
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                let if_var = Variable::new_with_type_indication(generate_built_in_variable_name_from_span(&element), Some(element.as_str().to_string()), TypeIndication::Bool);
                {
                    let mut if_var_write = if_var.write().unwrap();
                    if_var_write.set_code_location(CodeLocation::new_from_pest_rule(&element));
                }
                let mut output_if_write = output_if.write().unwrap();
                output_if_write.set_if_exp(if_var);
            }
            Rule::Scope_WithoutBracket => {
                let if_scope = output_if.read().unwrap().get_scope();
                parse_file::parse_Scope_WithoutBracket(element, if_scope.clone())?;
            }
            Rule::Elif => {
                let mut output_if_write = output_if.write().unwrap();
                let elif_blocks = output_if_write.get_elif_blocks();
                let current_elif_block_size = elif_blocks.len();
                let new_elif = parse_Elif(element, scope.clone())?;
                elif_blocks.insert(current_elif_block_size, new_elif);
            }
            Rule::Else => {
                let mut output_if_write = output_if.write().unwrap();
                let else_block = output_if_write.get_else_block();
                let new_else_block = parse_Else(element, scope.clone())?;
                *else_block = Some(new_else_block);
            }
            _ => unreachable!()
        }
    }
    {
        let mut output_if_write = output_if.write().unwrap();
        output_if_write.set_code_location(CodeLocation::new_from_pest_rule(&src));
    }
    let output_if_var = Variable::new_builtin(generate_built_in_variable_name_from_span(&src), TypedValue::If(output_if));
    {
        let mut output_if_var_write = output_if_var.write().unwrap();
        output_if_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src));
    }
    return Ok(output_if_var);
}

#[allow(non_snake_case)]
pub fn parse_Elif(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Elif, TydiLangError> {
    let mut output_elif = Elif::new(generate_built_in_variable_name_from_span(&src), scope.clone());
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                let elif_var = Variable::new_with_type_indication(generate_built_in_variable_name_from_span(&element), Some(element.as_str().to_string()), TypeIndication::Bool);
                {
                    let mut elif_var_write = elif_var.write().unwrap();
                    elif_var_write.set_code_location(CodeLocation::new_from_pest_rule(&element));
                }
                output_elif.set_elif_exp(elif_var);
            }
            Rule::Scope_WithoutBracket => {
                let elif_scope = output_elif.get_scope();
                parse_file::parse_Scope_WithoutBracket(element, elif_scope.clone())?;
            }
            _ => unreachable!()
        }
    }
    output_elif.set_code_location(CodeLocation::new_from_pest_rule(&src));
    return Ok(output_elif);
}

#[allow(non_snake_case)]
pub fn parse_Else(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Else, TydiLangError> {
    let mut output_else = Else::new(generate_built_in_variable_name_from_span(&src), scope.clone());
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Scope_WithoutBracket => {
                let else_scope = output_else.get_scope();
                parse_file::parse_Scope_WithoutBracket(element, else_scope.clone())?;
            }
            _ => unreachable!()
        }
    }
    output_else.set_code_location(CodeLocation::new_from_pest_rule(&src));
    return Ok(output_else);
}

#[allow(non_snake_case)]
pub fn parse_For(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let output_for = For::new(generate_built_in_variable_name_from_span(&src), scope.clone());
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                let for_var = element.as_str().to_string();
                let mut output_for_write = output_for.write().unwrap();
                output_for_write.set_for_var_name(for_var);
            }
            Rule::Exp => {
                let for_array_var = Variable::new_with_type_indication(generate_built_in_variable_name_from_span(&element), Some(element.as_str().to_string()), TypeIndication::Any);
                {
                    let mut for_array_var_write = for_array_var.write().unwrap();
                    for_array_var_write.set_code_location(CodeLocation::new_from_pest_rule(&element));
                }
                let mut output_for_write = output_for.write().unwrap();
                output_for_write.set_for_array_exp(for_array_var);
            }
            Rule::Scope_WithoutBracket => {
                let for_scope = output_for.read().unwrap().get_scope();
                parse_file::parse_Scope_WithoutBracket(element, for_scope.clone())?;
            }
            _ => unreachable!()
        }
    }
    {
        let mut output_for_write = output_for.write().unwrap();
        output_for_write.set_code_location(CodeLocation::new_from_pest_rule(&src));
    }
    let output_for_var = Variable::new_builtin(generate_built_in_variable_name_from_span(&src), TypedValue::For(output_for));
    {
        let mut output_for_var_write = output_for_var.write().unwrap();
        output_for_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src));
    }
    return Ok(output_for_var);
}