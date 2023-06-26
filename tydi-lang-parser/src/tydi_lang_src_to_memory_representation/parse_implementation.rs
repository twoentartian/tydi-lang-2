use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;

use crate::generate_name::{generate_init_value, generate_built_in_variable_name_from_span};
use crate::trait_common::HasDocument;
use crate::tydi_lang_src_to_memory_representation::parse_type::{parse_ArraySizeIndicator};
use crate::tydi_memory_representation::{Scope, GetScope, Variable, TraitCodeLocationAccess, CodeLocation, Implementation, Instance, Net, TypeIndication};
use crate::tydi_parser::*;

use crate::tydi_lang_src_to_memory_representation::{parse_template, parse_miscellaneous, parse_file};

#[allow(non_snake_case)]
pub fn parse_Implementation(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let mut output_implementation = Implementation::new_place_holder();
    let mut document = None;
    let mut name = generate_init_value();
    let mut template_args = None;
    let mut attributes = vec![];
    let mut streamlet_exp = String::new();
    let mut streamlet_exp_code_location = CodeLocation::new_unknown();

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
                template_args = parse_template::parse_TemplateArgs(element, scope.clone(), raw_src.clone())?;
            }
            Rule::Exp => {
                streamlet_exp = element.as_str().to_string();
                streamlet_exp_code_location = CodeLocation::new_from_pest_rule(&element, raw_src.clone());
            }
            Rule::ATTRIBUTE => {
                let attr = parse_miscellaneous::parse_ATTRIBUTE(element, scope.clone(), raw_src.clone())?;
                attributes.push(attr);
            }
            Rule::Scope_WithoutBracket => {
                output_implementation = Implementation::new(name.clone(), streamlet_exp.clone(), scope.clone());
                parse_file::parse_Scope_WithoutBracket(element, output_implementation.read().unwrap().get_scope(), raw_src.clone())?;
            }
            _ => unreachable!()
        }
    }
    {
        let mut output_implementation_write = output_implementation.write().unwrap();
        output_implementation_write.set_template_args(template_args);
        output_implementation_write.set_document(document);
        output_implementation_write.set_attributes(attributes);
        output_implementation_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        output_implementation_write.set_derived_streamlet_exp(streamlet_exp, streamlet_exp_code_location);
    }

    let output_implementation_var = Variable::new_implementation(name.clone(), output_implementation);
    {
        let mut output_implementation_var_write = output_implementation_var.write().unwrap();
        output_implementation_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
    }

    return Ok(output_implementation_var);
}

#[allow(non_snake_case)]
pub fn parse_Instance(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let mut document = None;
    let mut name = generate_init_value();
    let mut attributes = vec![];
    let mut implementation_exp = String::new();
    let mut implementation_exp_location = CodeLocation::new_unknown();
    let mut array_size_indicator = None;

    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::DOCUMENT_CONTENT => {
                document = Some(element.as_str().to_string());
            }
            Rule::ID => {
                name = element.as_str().to_string();
            }
            Rule::Exp => {
                implementation_exp = element.as_str().to_string();
                implementation_exp_location = CodeLocation::new_from_pest_rule(&element, raw_src.clone());
            }
            Rule::ArraySizeIndicator => {
                array_size_indicator = parse_ArraySizeIndicator(element.clone(), scope.clone(), raw_src.clone())?;
                if array_size_indicator.is_none() {
                    return Err(TydiLangError::new(format!("unknown array size indicator is not allowed for declaring instance"), CodeLocation::new_from_pest_rule(&element, raw_src.clone())));
                }
            }
            Rule::ATTRIBUTE => {
                let attr = parse_miscellaneous::parse_ATTRIBUTE(element, scope.clone(), raw_src.clone())?;
                attributes.push(attr);
            }
            _ => unreachable!()
        }
    }

    let output_instance = Instance::new(name.clone(), implementation_exp.clone());
    {
        let mut output_instance_write = output_instance.write().unwrap();
        output_instance_write.set_derived_implementation_exp(implementation_exp, implementation_exp_location);
        output_instance_write.set_document(document);
        output_instance_write.set_attributes(attributes);
        output_instance_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
    }

    let output_instance_var = Variable::new_instance(name.clone(), output_instance);
    {
        let mut output_instance_var_write = output_instance_var.write().unwrap();
        output_instance_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        output_instance_var_write.set_array_size(array_size_indicator);
    }

    return Ok(output_instance_var);
}

#[allow(non_snake_case)]
pub fn parse_Net(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let mut document = None;
    let name = generate_built_in_variable_name_from_span(&src);
    let mut net_name = None;
    let mut attributes = vec![];
    let mut source_var = Variable::new_place_holder();
    let mut sink_var = Variable::new_place_holder();

    let mut exp_index = 0;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::DOCUMENT_CONTENT => {
                document = Some(element.as_str().to_string());
            }
            Rule::Exp => {
                if exp_index == 0 {
                    source_var = Variable::new_with_type_indication(generate_built_in_variable_name_from_span(&element), Some(element.as_str().to_string().replace(" ", "")), TypeIndication::AnyPort);
                    {
                        let mut source_var_write = source_var.write().unwrap();
                        source_var_write.set_code_location(CodeLocation::new_from_pest_rule(&element, raw_src.clone()));
                    }
                } else if exp_index == 1 {
                    sink_var = Variable::new_with_type_indication(generate_built_in_variable_name_from_span(&element), Some(element.as_str().to_string().replace(" ", "")), TypeIndication::AnyPort);
                    {
                        let mut sink_var_write = sink_var.write().unwrap();
                        sink_var_write.set_code_location(CodeLocation::new_from_pest_rule(&element, raw_src.clone()));
                    }
                } else {
                    unreachable!();
                }
                exp_index += 1;
            }
            Rule::NetName => {
                let parsed_name = parse_Netname(element, scope.clone(), raw_src.clone())?;
                match parsed_name {
                    Some(v) => net_name = Some(v),
                    None => net_name = None,
                }
            }
            Rule::ATTRIBUTE => {
                let attr = parse_miscellaneous::parse_ATTRIBUTE(element, scope.clone(), raw_src.clone())?;
                attributes.push(attr);
            }
            _ => unreachable!()
        }
    }

    let output_net = Net::new(name.clone());
    {
        let mut output_net_write = output_net.write().unwrap();
        output_net_write.set_source(source_var);
        output_net_write.set_sink(sink_var);
        output_net_write.set_net_name(net_name);
        output_net_write.set_document(document);
        output_net_write.set_attributes(attributes);
        output_net_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
    }
    let output_net_var = Variable::new_net(name.clone(), output_net);
    {
        let mut output_net_var_write = output_net_var.write().unwrap();
        output_net_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
    }

    return Ok(output_net_var);
}

#[allow(non_snake_case)]
pub fn parse_Netname(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Option<Arc<RwLock<Variable>>>, TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                let output_var = Variable::new_with_type_indication(generate_built_in_variable_name_from_span(&element), Some(element.as_str().to_string()), TypeIndication::String);
                {
                    let mut output_var_write = output_var.write().unwrap();
                    output_var_write.set_code_location(CodeLocation::new_from_pest_rule(&element, raw_src.clone()));
                }
                return Ok(Some(output_var));
            }
            _ => unreachable!()
        }
    }
    return Ok(None);
}