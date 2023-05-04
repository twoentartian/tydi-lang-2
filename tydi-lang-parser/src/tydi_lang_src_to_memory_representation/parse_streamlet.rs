use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;

use crate::generate_name::generate_init_value;
use crate::trait_common::HasDocument;
use crate::tydi_memory_representation::{Scope, Streamlet, GetScope, Variable, TypedValue, TraitCodeLocationAccess, CodeLocation};
use crate::tydi_parser::*;

use crate::tydi_lang_src_to_memory_representation::{parse_template, parse_miscellaneous, parse_file};

#[allow(non_snake_case)]
pub fn parse_StreamLet(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), TydiLangError> {
    let mut output_streamlet = Streamlet::new_place_holder();
    let mut document = None;
    let mut name = generate_init_value();
    let mut template_args = None;
    let mut attributes = vec![];

    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::DOCUMENT_CONTENT => {
                document = Some(element.as_str().to_string());
            }
            Rule::ID => {
                name = element.as_str().to_string();
            }
            Rule::TemplateArgs => {
                template_args = parse_template::parse_TemplateArgs(element, scope.clone())?;
            }
            Rule::ATTRIBUTE => {
                let attr = parse_miscellaneous::parse_ATTRIBUTE(element, scope.clone())?;
                attributes.push(attr);
            }
            Rule::Scope_WithoutBracket => {
                output_streamlet = Streamlet::new(name.clone(), scope.clone());
                parse_file::parse_Scope_WithoutBracket(element, output_streamlet.read().unwrap().get_scope())?;
            }
            _ => unreachable!()
        }
    }
    {
        let mut output_streamlet_write = output_streamlet.write().unwrap();
        output_streamlet_write.set_template_args(template_args);
        output_streamlet_write.set_document(document);
        output_streamlet_write.set_attributes(attributes);
        output_streamlet_write.set_code_location(CodeLocation::new_from_pest_rule(&src));
    }

    let output_streamlet_var = Variable::new_streamlet(name.clone(), output_streamlet);
    {
        let mut output_streamlet_var_write = output_streamlet_var.write().unwrap();
        output_streamlet_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src));
    }
    {
        let mut scope_write = scope.write().unwrap();
        scope_write.add_var(output_streamlet_var)?;
    }

    return Ok(());
}