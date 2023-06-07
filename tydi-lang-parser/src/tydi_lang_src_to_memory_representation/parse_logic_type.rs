use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::generate_name::generate_init_value;
use crate::trait_common::HasDocument;
use crate::tydi_lang_src_to_memory_representation::{parse_template::parse_TemplateArgs, parse_file::parse_Scope_WithoutBracket};
use crate::tydi_memory_representation::{Scope, LogicType, LogicBit, Variable, TraitCodeLocationAccess, CodeLocation, GetScope, LogicGroup, LogicUnion, LogicStream, LogicStreamProperty};
use crate::{tydi_parser::*, generate_name};

#[allow(non_snake_case)]
pub fn parse_LogicalBit(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let mut bit_exp = generate_name::generate_init_value();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                bit_exp = element.as_str().to_string();
            }
            _ => unreachable!()
        }
    }
    let output_logic_bit = LogicBit::new(generate_name::generate_built_in_variable_name_from_span(&src), Some(bit_exp));
    {
        let mut output_logic_bit_write = output_logic_bit.write().unwrap();
        output_logic_bit_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
    }
    let logic_bit_var = Variable::new_logic_type(generate_name::generate_built_in_variable_name_from_span(&src), Arc::new(RwLock::new(LogicType::LogicBitType(output_logic_bit))));
    {
        let mut logic_bit_var_write = logic_bit_var.write().unwrap();
        let code_location = CodeLocation::new_from_pest_rule(&src, raw_src.clone());
        logic_bit_var_write.set_code_location(code_location);
    }
    {
        let mut scope_write = scope.write().unwrap();
        scope_write.add_var(logic_bit_var.clone())?;
    }
    
    return Ok(logic_bit_var);
}

#[allow(non_snake_case)]
pub fn parse_LogicalGroup(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let mut output_logic_group = LogicGroup::new_place_holder();
    let mut document: Option<String> = None;
    let mut group_name = generate_name::generate_init_value();
    let mut template_args = None;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::DOCUMENT_CONTENT => {
                document = Some(element.as_str().to_string());
            }
            Rule::ID => {
                group_name = element.as_str().to_string();
            }
            Rule::TemplateArgs => {
                template_args = parse_TemplateArgs(element, scope.clone(), raw_src.clone())?;
            }
            Rule::Scope_WithoutBracket => {
                output_logic_group = LogicGroup::new(generate_name::generate_built_in_variable_name_from_span(&src), scope.clone());
                let output_logic_group_read = output_logic_group.read().unwrap();
                let output_logic_scope = output_logic_group_read.get_scope();
                parse_Scope_WithoutBracket(element, output_logic_scope.clone(), raw_src.clone())?;
            }
            _ => unreachable!()
        }
    }
    {
        let mut output_logic_group_write = output_logic_group.write().unwrap();
        output_logic_group_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        output_logic_group_write.set_document(document);
        output_logic_group_write.set_template_args(template_args);
    }

    let logic_group_var = Variable::new_logic_type(group_name.clone(), Arc::new(RwLock::new(LogicType::LogicGroupType(output_logic_group))));
    {
        let mut logic_group_var_write = logic_group_var.write().unwrap();
        logic_group_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
    }

    return Ok(logic_group_var);
}

#[allow(non_snake_case)]
pub fn parse_LogicalUnion(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let mut output_logic_union = LogicUnion::new_place_holder();
    let mut document: Option<String> = None;
    let mut union_name = generate_name::generate_init_value();
    let mut template_args = None;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::DOCUMENT_CONTENT => {
                document = Some(element.as_str().to_string());
            }
            Rule::ID => {
                union_name = element.as_str().to_string();
            }
            Rule::TemplateArgs => {
                template_args = parse_TemplateArgs(element, scope.clone(), raw_src.clone())?;
            }
            Rule::Scope_WithoutBracket => {
                output_logic_union = LogicUnion::new(generate_name::generate_built_in_variable_name_from_span(&src), scope.clone());
                let output_logic_union_read = output_logic_union.read().unwrap();
                let output_logic_scope = output_logic_union_read.get_scope();
                parse_Scope_WithoutBracket(element, output_logic_scope.clone(), raw_src.clone())?;
            }
            _ => unreachable!()
        }
    }
    {
        let mut output_logic_union_write = output_logic_union.write().unwrap();
        output_logic_union_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        output_logic_union_write.set_document(document);
        output_logic_union_write.set_template_args(template_args);
    }

    let logic_union_var = Variable::new_logic_type(union_name.clone(), Arc::new(RwLock::new(LogicType::LogicUnionType(output_logic_union))));
    {
        let mut logic_union_var_write = logic_union_var.write().unwrap();
        logic_union_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
    }

    return Ok(logic_union_var);
}

#[allow(non_snake_case)]
pub fn parse_StreamProperty(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let mut name = generate_init_value();
    let mut exp = generate_init_value();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                name = element.as_str().to_string();
            }
            Rule::Exp => {
                exp = element.as_str().to_string();
            }
            _ => unreachable!()
        }
    }
    let stream_property = LogicStreamProperty::try_from(name.clone());
    if stream_property.is_err() {
        return Err(TydiLangError::new(format!("unknown stream property: {}", name), CodeLocation::new_from_pest_rule(&src, raw_src.clone())));
    }
    let stream_property = stream_property.ok().unwrap();
    let output_var = Variable::new(stream_property.get_full_name().to_string(), Some(exp));
    {
        let mut output_var_write = output_var.write().unwrap();
        output_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
    }
    return Ok(output_var);
}

#[allow(non_snake_case)]
pub fn parse_LogicalStream(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<String>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let mut stream_properties = vec![];
    let mut stream_type_exp: String = generate_init_value();
    let mut stream_type_location = CodeLocation::new_unknown();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                stream_type_exp = element.as_str().to_string();
                stream_type_location = CodeLocation::new_from_pest_rule(&src, raw_src.clone());
            }
            Rule::StreamProperty => {
                let property = parse_StreamProperty(element, scope.clone(), raw_src.clone())?;
                stream_properties.push(property);
            }
            _ => unreachable!()
        }
    }
    let output_logic_stream = LogicStream::new(generate_name::generate_built_in_variable_name_from_span(&src), Some(stream_type_exp));
    let logic_stream_var_name = generate_name::generate_built_in_variable_name_from_span(&src);
    // add stream property
    {
        let mut output_logic_stream_write = output_logic_stream.write().unwrap();
        {
            let output_logic_stream_type = output_logic_stream_write.get_stream_type();
            let mut output_logic_stream_type_write = output_logic_stream_type.write().unwrap();
            output_logic_stream_type_write.set_code_location(stream_type_location);
        }

        output_logic_stream_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        for stream_property in stream_properties {
            let result = output_logic_stream_write.apply_property_var(stream_property);
            if result.is_err() {
                return Err(TydiLangError::new(result.err().unwrap(), CodeLocation::new_from_pest_rule(&src, raw_src.clone())));
            }
        }
    }
    // set stream variable
    let logic_stream_var = Variable::new_logic_type(logic_stream_var_name, Arc::new(RwLock::new(LogicType::LogicStreamType(output_logic_stream))));
    {
        let mut logic_stream_var_write = logic_stream_var.write().unwrap();
        logic_stream_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
    }
    //write var to scope
    {
        let mut scope_write = scope.write().unwrap();
        scope_write.add_var(logic_stream_var.clone())?;
    }

    return Ok(logic_stream_var);
}
