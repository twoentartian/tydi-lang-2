use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use crate::error::TydiLangError;
use crate::tydi_memory_representation::{Scope, TemplateArg, TraitCodeLocationAccess, CodeLocation};
use crate::tydi_parser::*;

use super::parse_type::parse_AllTypeKeyword;

pub fn parse_TemplateArgs(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Option<BTreeMap<usize, TemplateArg>>, TydiLangError> {
    let mut arg_container: BTreeMap<usize, TemplateArg> = BTreeMap::new();
    let mut index = 0;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Arg => {
                let arg = parse_Arg(element, scope.clone())?;
                arg_container.insert(index, arg);
                index += 1;
            }
            _ => todo!()
        }
    }

    if arg_container.is_empty() {
        return Ok(None);
    }
    else {
        return Ok(Some(arg_container));
    }
}

pub fn parse_Arg(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<TemplateArg, TydiLangError> {
    let mut output = TemplateArg::new_place_holder();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Arg_Common => {
                output = parse_Arg_Common(element, scope.clone())?;
            }
            _ => todo!()
        }
    }
    return Ok(output);
}

pub fn parse_Arg_Common(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<TemplateArg, TydiLangError> {
    let mut output_arg = TemplateArg::new_place_holder();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                output_arg.set_name(element.as_str().to_string());
            }
            Rule::AllTypeKeyword => {
                let (type_indication, is_array) = parse_AllTypeKeyword(element, scope.clone())?;
                output_arg.set_is_array(is_array);
                output_arg.set_type_indication(type_indication);
            }
            _ => todo!()
        }
    }
    output_arg.set_code_location(CodeLocation::new_from_pest_rule(&src));
    return Ok(output_arg);
}