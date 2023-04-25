use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::trait_common::GetScope;
use crate::tydi_memory_representation::{Package, Scope};
use crate::tydi_parser::*;

pub fn parse_PackageStatement(src: Pair<Rule>, package: Arc<RwLock<Package>>) -> Result<Arc<RwLock<Package>>, TydiLangError> {
    for element in src.into_inner().into_iter() {
        match element.as_rule() {
            Rule::ID => {
                package.write().unwrap().set_name( element.as_str().to_string());
            }
            _ => todo!()
        }
    }
    return Ok(package);
}

pub fn parse_Scope_WithoutBracket(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<(), TydiLangError> {
    use crate::tydi_lang_src_to_memory_representation::parse_statement::*;

    for element in src.into_inner().into_iter() {
        match element.as_rule() {
            Rule::StatementDeclareVariable => {
                parse_StatementDeclareVariable(element, scope.clone())?;
            }
            Rule::StatementDeclareType => {
                parse_StatementDeclareType(element, scope.clone())?;
            }
            Rule::StatementDeclareGroup => {
                parse_StatementDeclareGroup(element, scope.clone())?;
            }
            Rule::StatementDeclareUnion => {
                todo!()
            }
            Rule::StatementDeclarePort => {
                todo!()
            }
            Rule::StatementDeclareImplementation => {
                todo!()
            }
            Rule::StatementDeclareIf => {
                todo!()
            }
            Rule::StatementDeclareFor => {
                todo!()
            }
            Rule::StatementUsePackage => {
                todo!()
            }
            Rule::StatementFunction => {
                todo!()
            }
            _ => todo!()
        }
    }
    return Ok(());
}