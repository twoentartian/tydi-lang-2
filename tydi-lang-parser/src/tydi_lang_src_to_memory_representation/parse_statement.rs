use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::trait_common::GetName;
use crate::tydi_memory_representation::{CodeLocation, Scope, SrcInfo, TraitCodeLocationAccess, TypeIndication, Variable};
use crate::{tydi_parser::*, generate_name};
use crate::tydi_lang_src_to_memory_representation::{parse_type, parse_logic_type, parse_streamlet, parse_implementation};

use super::parse_logic_flow::{parse_If, parse_For};


#[allow(non_snake_case)]
pub fn parse_StatementDeclareVariable(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    let var = Variable::new_place_holder();
    let mut type_indicator = TypeIndication::Any;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                var.write().unwrap().set_name(element.as_str().to_string());
            }
            Rule::TypeIndicator => {
                type_indicator = parse_type::parse_TypeIndicator(element, scope.clone(), raw_src.clone())?;
            }
            Rule::Exp => {
                var.write().unwrap().set_exp(Some(element.as_str().to_string()));
            }
            _ => unreachable!()
        }
    }
    {
        let mut var_write = var.write().unwrap();
        var_write.set_type_indication(type_indicator.clone());
        var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        var_write.set_is_name_user_defined(true);
    }
    {
        let mut scope_write = scope.write().unwrap();
        scope_write.add_var(var)?;
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementDeclareType(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    let var = Variable::new_place_holder();
    let mut type_indicator = TypeIndication::Any;
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::ID => {
                var.write().unwrap().set_name(element.as_str().to_string());
            }
            Rule::TypeIndicator => {
                type_indicator = parse_type::parse_TypeIndicator(element, scope.clone(), raw_src.clone())?;
            }
            _ => unreachable!()
        }
    }
    
    {
        let mut var_write = var.write().unwrap();
        var_write.set_type_indication(type_indicator.clone());
        var_write.set_is_property_of_scope(true);
        var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
        var_write.set_is_name_user_defined(true);

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

#[allow(non_snake_case)]
pub fn parse_StatementDeclareGroup(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::LogicalGroup => {
                let var = parse_logic_type::parse_LogicalGroup(element, scope.clone(), raw_src.clone())?;
                {
                    let mut scope_write = scope.write().unwrap();
                    scope_write.add_var(var)?;
                }
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementDeclareUnion(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::LogicalUnion => {
                let var = parse_logic_type::parse_LogicalUnion(element, scope.clone(), raw_src.clone())?;
                {
                    let mut scope_write = scope.write().unwrap();
                    scope_write.add_var(var)?;
                }
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementDeclareStreamlet(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::StreamLet => {
                let var = parse_streamlet::parse_StreamLet(element, scope.clone(), raw_src.clone())?;
                {
                    let mut scope_write = scope.write().unwrap();
                    scope_write.add_var(var)?;
                }
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementDeclarePort(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Port => {
                let port_var = parse_streamlet::parse_Port(element, scope.clone(), raw_src.clone())?;
                {
                    let mut scope_write = scope.write().unwrap();
                    scope_write.add_var(port_var)?;
                }
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementDeclareImplementation(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Implementation => {
                let var = parse_implementation::parse_Implementation(element, scope.clone(), raw_src.clone())?;
                {
                    let mut scope_write = scope.write().unwrap();
                    scope_write.add_var(var)?;
                }
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementDeclareInstance(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Instance => {
                let var = parse_implementation::parse_Instance(element, scope.clone(), raw_src.clone())?;
                {
                    let mut scope_write = scope.write().unwrap();
                    scope_write.add_var(var)?;
                }
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementDeclareNet(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::Net => {
                let var = parse_implementation::parse_Net(element, scope.clone(), raw_src.clone())?;
                {
                    let mut scope_write = scope.write().unwrap();
                    scope_write.add_var(var)?;
                }
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementFunction(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::FunctionExp => {
                let function_exp = element.as_str().to_string();
                let function_var_name = generate_name::generate_built_in_variable_name_from_span(&element);
                let function_var = Variable::new_function_var(function_var_name, function_exp);
                {
                    let mut function_var_write = function_var.write().unwrap();
                    function_var_write.set_code_location(CodeLocation::new_from_pest_rule(&element, raw_src.clone()));
                }
                scope.write().unwrap().add_var(function_var)?;
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementIf(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::If => {
                let var = parse_If(element, scope.clone(), raw_src.clone())?;
                {
                    let mut scope_write = scope.write().unwrap();
                    scope_write.add_var(var)?;
                }
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementFor(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        let rule = element.as_rule();
        match rule {
            Rule::For => {
                let var = parse_For(element, scope.clone(), raw_src.clone())?;
                {
                    let mut scope_write = scope.write().unwrap();
                    scope_write.add_var(var)?;
                }
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}