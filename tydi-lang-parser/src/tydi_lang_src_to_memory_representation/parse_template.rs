use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use crate::error::TydiLangError;
use crate::tydi_memory_representation::{Scope, TemplateArg, TraitCodeLocationAccess, CodeLocation, SrcInfo};
use crate::tydi_parser::*;

use super::parse_type::parse_AllTypeKeyword;

#[allow(non_snake_case)]
pub fn parse_TemplateArgs(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<Option<BTreeMap<usize, TemplateArg>>, TydiLangError> {
    let mut arg_container: BTreeMap<usize, TemplateArg> = BTreeMap::new();
    let mut index = 0;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Arg => {
                let arg = parse_Arg(element, scope.clone(), raw_src.clone())?;
                arg_container.insert(index, arg);
                index += 1;
            }
            _ => unreachable!()
        }
    }

    if arg_container.is_empty() {
        return Ok(None);
    }
    else {
        return Ok(Some(arg_container));
    }
}

#[allow(non_snake_case)]
pub fn parse_Arg(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<TemplateArg, TydiLangError> {
    let mut output = TemplateArg::new_place_holder();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Arg_Common => {
                output = parse_Arg_Common(element, scope.clone(), raw_src.clone())?;
            }
            _ => unreachable!()
        }
    }
    return Ok(output);
}

#[allow(non_snake_case)]
pub fn parse_Arg_Common(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<TemplateArg, TydiLangError> {
    let mut output_arg = TemplateArg::new_place_holder();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                output_arg.set_name(element.as_str().to_string());
            }
            Rule::AllTypeKeyword => {
                let (type_indication, is_array) = parse_AllTypeKeyword(element, scope.clone(), raw_src.clone())?;
                output_arg.set_is_array(is_array);
                output_arg.set_type_indication(type_indication);
            }
            _ => unreachable!()
        }
    }
    output_arg.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
    return Ok(output_arg);
}