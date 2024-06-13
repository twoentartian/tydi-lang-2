use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::tydi_memory_representation::{CodeLocation, Package, Scope, SrcInfo, TraitCodeLocationAccess, TypeIndication, Variable};
use crate::tydi_parser::*;

#[allow(non_snake_case)]
pub fn parse_PackageStatement(src: Pair<Rule>, package: Arc<RwLock<Package>>, _: Arc<SrcInfo>) -> Result<Arc<RwLock<Package>>, TydiLangError> {
    for element in src.into_inner().into_iter() {
        match element.as_rule() {
            Rule::ID => {
                package.write().unwrap().set_name( element.as_str().to_string());
            }
            _ => unreachable!()
        }
    }
    return Ok(package);
}

#[allow(non_snake_case)]
pub fn parse_Scope_WithoutBracket(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    use crate::tydi_lang_src_to_memory_representation::parse_statement::*;

    for element in src.into_inner().into_iter() {
        match element.as_rule() {
            Rule::StatementDeclareVariable => {
                parse_StatementDeclareVariable(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementDeclareType => {
                parse_StatementDeclareType(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementDeclareGroup => {
                parse_StatementDeclareGroup(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementDeclareUnion => {
                parse_StatementDeclareUnion(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementDeclareStreamlet => {
                parse_StatementDeclareStreamlet(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementDeclarePort => {
                parse_StatementDeclarePort(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementDeclareImplementation => {
                parse_StatementDeclareImplementation(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementDeclareInstance => {
                parse_StatementDeclareInstance(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementDeclareNet => {
                parse_StatementDeclareNet(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementDeclareIf => {
                parse_StatementIf(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementDeclareFor => {
                parse_StatementFor(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementUsePackage => {
                parse_StatementUsePackage(element, scope.clone(), raw_src.clone())?;
            }
            Rule::StatementFunction => {
                parse_StatementFunction(element, scope.clone(), raw_src.clone())?;
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}

#[allow(non_snake_case)]
pub fn parse_StatementUsePackage(src: Pair<Rule>, scope: Arc<RwLock<Scope>>, raw_src: Arc<SrcInfo>) -> Result<(), TydiLangError> {
    for element in src.clone().into_inner().into_iter() {
        match element.as_rule() {
            Rule::ID => {
                let package_name = element.as_str().to_string();
                let package_ref_var = Variable::new_with_type_indication(package_name.clone(), Some(package_name.clone()), TypeIndication::PackageReference);
                {
                    let mut package_ref_var_write = package_ref_var.write().unwrap();
                    package_ref_var_write.set_code_location(CodeLocation::new_from_pest_rule(&src, raw_src.clone()));
                }
                {
                    let mut scope_write = scope.write().unwrap();
                    scope_write.add_var(package_ref_var)?;
                }
            }
            _ => unreachable!()
        }
    }
    return Ok(());
}