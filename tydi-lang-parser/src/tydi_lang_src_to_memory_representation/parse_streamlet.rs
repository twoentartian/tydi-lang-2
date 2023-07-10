use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;

use crate::generate_name::{generate_init_value, generate_built_in_variable_name_from_span};
use crate::trait_common::HasDocument;
use crate::tydi_lang_src_to_memory_representation::parse_type::{parse_LogicalType, parse_ArraySizeIndicator};
use crate::tydi_memory_representation::{Scope, Streamlet, GetScope, Variable, TraitCodeLocationAccess, CodeLocation, Port, TypeIndication, PortDirection, GlobalIdentifier};
use crate::tydi_parser::*;

use crate::tydi_lang_src_to_memory_representation::{parse_template, parse_miscellaneous, parse_file};

#[allow(non_snake_case)]
pub fn parse_StreamLet(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
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
                template_args = parse_template::parse_TemplateArgs(element, scope.clone(), raw_src.clone())?;
            }
            Rule::ATTRIBUTE => {
                let attr = parse_miscellaneous::parse_ATTRIBUTE(element, scope.clone(), raw_src.clone())?;
                attributes.push(attr);
            }
            Rule::Scope_WithoutBracket => {
                output_streamlet = Streamlet::new(name.clone(), scope.clone());
                parse_file::parse_Scope_WithoutBracket(element, output_streamlet.read().unwrap().get_scope(), raw_src.clone())?;
            }
            _ => unreachable!()
        }
    }
    {
        let mut output_streamlet_write = output_streamlet.write().unwrap();
        output_streamlet_write.set_template_args(template_args);
        output_streamlet_write.set_document(document);
        output_streamlet_write.set_attributes(attributes);
        output_streamlet_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        output_streamlet_write.set_parent_scope(Some(scope.clone()));
        output_streamlet_write.set_id_in_scope(Some(name.clone()));
    }

    let output_streamlet_var = Variable::new_streamlet(name.clone(), output_streamlet);
    {
        let mut output_streamlet_var_write = output_streamlet_var.write().unwrap();
        output_streamlet_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        output_streamlet_var_write.set_is_name_user_defined(true);
    }

    return Ok(output_streamlet_var);
}

#[allow(non_snake_case)]
pub fn parse_Port(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let mut document = None;
    let mut name = generate_init_value();
    let mut attributes = vec![];
    let mut port_logical_type_exp = Variable::new_place_holder();
    let mut port_time_domain_exp = None;
    let mut array_size_indicator = None;
    let mut port_driection = PortDirection::Unknown;
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
                port_logical_type_exp = Variable::new_with_type_indication(generate_built_in_variable_name_from_span(&element), Some(element.as_str().to_string()), TypeIndication::AnyLogicType);
                {
                    let mut port_logical_type_exp_write = port_logical_type_exp.write().unwrap();
                    port_logical_type_exp_write.set_code_location(CodeLocation::new_from_pest_rule(&element, raw_src.clone()));
                }
            }
            Rule::PortDirection => {
                port_driection = parse_PortDirection(element, scope.clone(), raw_src.clone())?;
            }
            Rule::ArraySizeIndicator => {
                array_size_indicator = parse_ArraySizeIndicator(element, scope.clone(), raw_src.clone())?;
            }
            Rule::PortTimeDomain => {
                port_time_domain_exp = parse_PortTimeDomain(element.clone(), scope.clone(), raw_src.clone())?;
                {
                    if port_time_domain_exp.is_some() {
                        let port_time_domain_exp_unwrap = port_time_domain_exp.clone().unwrap();
                        let mut port_time_domain_exp_write = port_time_domain_exp_unwrap.write().unwrap();
                        port_time_domain_exp_write.set_code_location(CodeLocation::new_from_pest_rule(&element, raw_src.clone()));
                    }
                }
            }
            Rule::ATTRIBUTE => {
                let attr = parse_miscellaneous::parse_ATTRIBUTE(element, scope.clone(), raw_src.clone())?;
                attributes.push(attr);
            }
            _ => unreachable!()
        }
    }

    let output_port = Port::new(generate_built_in_variable_name_from_span(&src), port_driection, port_logical_type_exp);
    {
        let mut output_port_write = output_port.write().unwrap();
        output_port_write.set_attributes(attributes);
        output_port_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        output_port_write.set_document(document);
        output_port_write.set_parent_scope(Some(scope.clone()));
        output_port_write.set_id_in_scope(Some(name.clone()));
        match port_time_domain_exp {
            Some(time_domain_var) => output_port_write.set_time_domain(time_domain_var),
            None => output_port_write.set_time_domain(Port::get_default_time_domain()),
        }
    }
    let output_port_var = Variable::new_port(name.clone(), output_port);
    {
        let mut output_port_var_write = output_port_var.write().unwrap();
        output_port_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        output_port_var_write.set_is_property_of_scope(true);
        match array_size_indicator {
            Some(array_size_var) => {
                output_port_var_write.set_array_size(Some(array_size_var));
            },
            None => {
                output_port_var_write.set_array_size(None);
            }
        }
        output_port_var_write.set_is_name_user_defined(true);
    }
    return Ok(output_port_var);
}

#[allow(non_snake_case)]
pub fn parse_PortTimeDomain(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Option<Arc<RwLock<Variable>>>, TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                let output = Variable::new_with_type_indication(generate_built_in_variable_name_from_span(&src), Some(element.as_str().to_string()), TypeIndication::String);
                return Ok(Some(output));
            }
            _ => unreachable!()
        }
    }
    return Ok(None);
}

#[allow(non_snake_case)]
pub fn parse_PortDirection(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<PortDirection, TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::PortDirectionDirIn => return Ok(PortDirection::In),
            Rule::PortDirectionDirOut => return Ok(PortDirection::Out),
            _ => unreachable!()
        }
    }
    unreachable!()
}