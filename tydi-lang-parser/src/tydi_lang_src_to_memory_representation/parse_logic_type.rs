use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::trait_common::HasDocument;
use crate::tydi_lang_src_to_memory_representation::parse_template::parse_TemplateArgs;
use crate::tydi_memory_representation::{Scope, LogicType, LogicBit, Variable, TypeIndication, TypedValue, TraitCodeLocationAccess, CodeLocation, GetScope, LogicGroup, template_args};
use crate::{tydi_parser::*, generate_name};

use super::parse_file::parse_Scope_WithoutBracket;

pub fn parse_LogicalBit(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let mut bit_exp = generate_name::generate_init_value();
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Exp => {
                bit_exp = element.as_str().to_string();
            }
            _ => todo!()
        }
    }
    let output_logic_bit = LogicBit::new(generate_name::generate_built_in_variable_name_from_span(&src), Some(bit_exp));
    let logic_bit_var_name = generate_name::generate_built_in_variable_name_from_span(&src);
    let logic_bit_var = Variable::new(logic_bit_var_name, None);
    {
        let mut logic_bit_var_write = logic_bit_var.write().unwrap();
        logic_bit_var_write.set_type_indication(TypeIndication::AnyLogicType);

        let value = TypedValue::LogicTypeValue(Arc::new(RwLock::new(LogicType::LogicBitType(output_logic_bit))));
        logic_bit_var_write.set_value(vec![value]);         // we should only set value if it is a logic type
        
        let code_location = CodeLocation::new_from_pest_rule(&src);
        logic_bit_var_write.set_code_location(code_location);
    }
    {
        let mut scope_write = scope.write().unwrap();
        scope_write.add_var(logic_bit_var.clone())?;
    }
    
    return Ok(logic_bit_var);
}

pub fn parse_LogicalGroup(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
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
                template_args = parse_TemplateArgs(element, scope.clone())?;
            }
            Rule::Scope_WithoutBracket => {
                output_logic_group = LogicGroup::new(generate_name::generate_built_in_variable_name_from_span(&src), scope.clone());
                let output_logic_group_read = output_logic_group.read().unwrap();
                let output_logic_scope = output_logic_group_read.get_scope();
                parse_Scope_WithoutBracket(element, output_logic_scope.clone())?;
            }
            _ => todo!()
        }
    }
    {
        let mut output_logic_group_write = output_logic_group.write().unwrap();
        output_logic_group_write.set_code_location(CodeLocation::new_from_pest_rule(&src));
        output_logic_group_write.set_document(document);
        output_logic_group_write.set_template_args(template_args);
    }

    let logic_group_var = Variable::new(group_name.clone(), None);
    {
        let mut logic_group_var_write = logic_group_var.write().unwrap();
        logic_group_var_write.set_value(vec![TypedValue::LogicTypeValue(Arc::new(RwLock::new(LogicType::LogicGroupType(output_logic_group))))]);
        logic_group_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src));
        logic_group_var_write.set_type_indication(TypeIndication::AnyLogicType);
    }

    return Ok(logic_group_var);
}