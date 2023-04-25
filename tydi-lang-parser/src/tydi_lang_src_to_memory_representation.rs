use std::sync::{Arc, RwLock};

mod parse_file;
use parse_file::*;

mod parse_statement;
use parse_statement::*;

mod parse_type;
use parse_type::*;

mod parse_logic_type;
use parse_logic_type::*;

mod parse_var;
use parse_var::*;

mod parse_template;
use parse_template::*;

use crate::error::TydiLangError;
use crate::trait_common::GetScope;
use crate::tydi_parser::*;
use crate::tydi_memory_representation::{Scope, CodeLocation, Package, TraitCodeLocationAccess};

pub fn tydi_lang_src_to_memory_representation(src: String) -> Result<Arc<RwLock<Package>>, TydiLangError> {
    let src_pointer = Arc::new(RwLock::new(src.clone()));
    let parse_result = TydiLangSrc::parse(Rule::TydiFile,&src);
    if parse_result.is_err() {
        let parse_result = parse_result.err().unwrap();
        match parse_result.variant {
            pest::error::ErrorVariant::ParsingError { positives, negatives } => {
                let error_location = match parse_result.location {
                    pest::error::InputLocation::Pos(begin) => CodeLocation::new_only_begin(begin),
                    pest::error::InputLocation::Span((begin, end)) => CodeLocation::new(begin, end),
                };
                let message_from_parser = format!("Expected: {:?}, found: {:?}", positives, negatives);
                return Err(TydiLangError { 
                    message: format!("cannot parse the source code, message from parser: {}", message_from_parser), 
                    location: error_location, 
                });
            },
            pest::error::ErrorVariant::CustomError { message } => {
                return Err(TydiLangError { 
                    message: format!("cannot parse the source code, message from parser: {}", message), 
                    location: CodeLocation::new_unknown(), 
                });
            },
        }
    }

    let mut package_name = String::new();
    let parse_result = parse_result.ok().unwrap();
    let output_package = Package::new(package_name);
    for element in parse_result.clone().into_iter() {
        match element.as_rule() {
            Rule::PackageStatement => {
                parse_PackageStatement(element, output_package.clone())?;
            }
            Rule::Scope_WithoutBracket => {
                parse_Scope_WithoutBracket(element, output_package.read().unwrap().get_scope())?;
            }
            Rule::EOI => {
                //do nothing
            }
            _ => todo!()
        }
    }
    let loc = CodeLocation::new(0, src.len());
    output_package.write().unwrap().set_code_location(loc);
    return Ok(output_package);
}

#[cfg(test)]
mod test_tydi_lang_src_to_memory_representation {
    use super::*;
    use serde_json::Value;

    fn print_tydi_lang_error(error: &TydiLangError, src_ptr: Option<Arc<RwLock<String>>>) {
        println!("{}", error.message);
        println!("{}", error.location.show(src_ptr));
    }

    #[test]
    fn simple_package() {
        let src = String::from(r#"
        package test;
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            print_tydi_lang_error(&result, src_ptr.clone());
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        println!("{}", result.read().unwrap().get_code_location().show(src_ptr));
    }

    #[test]
    fn simple_declare_variable_0() {
        let src = String::from(r#"
        package test;
        i = 10;
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            print_tydi_lang_error(&result, src_ptr.clone());
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            let variable_i = &target["package_scope"]["variables"]["i"];
            assert_eq!(variable_i["name"], format!("i"));
            assert_eq!(variable_i["exp"], format!("10"));
            assert_eq!(variable_i["evaluated"], format!("NotEvaluated"));
            assert_eq!(variable_i["type_indication"], format!("Any"));
            assert_eq!(variable_i["is_array"], false);
        }
    }

    #[test]
    fn simple_declare_variable_1() {
        let src = String::from(r#"
        package test;
        i:int = 10;
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            print_tydi_lang_error(&result, src_ptr.clone());
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            let variable_i = &target["package_scope"]["variables"]["i"];
            assert_eq!(variable_i["name"], format!("i"));
            assert_eq!(variable_i["exp"], format!("10"));
            assert_eq!(variable_i["evaluated"], format!("NotEvaluated"));
            assert_eq!(variable_i["type_indication"], format!("Int"));
            assert_eq!(variable_i["is_array"], false);
        }
    }

    #[test]
    fn simple_declare_variable_2() {
        let src = String::from(r#"
        package test;
        i:[int] = {10, 20, 30};
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            print_tydi_lang_error(&result, src_ptr.clone());
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            let variable_i = &target["package_scope"]["variables"]["i"];
            assert_eq!(variable_i["name"], format!("i"));
            assert_eq!(variable_i["exp"], format!("{{10, 20, 30}}"));
            assert_eq!(variable_i["evaluated"], format!("NotEvaluated"));
            assert_eq!(variable_i["type_indication"], format!("Int"));
            assert_eq!(variable_i["is_array"], true);
        }
    }

    #[test]
    fn simple_declare_variable_3() {
        let src = String::from(r#"
        package test;
        i:[int] = {10, 20, 30};
        i0 = i[0] + func(i);
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            print_tydi_lang_error(&result, src_ptr.clone());
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            let variable_i = &target["package_scope"]["variables"]["i0"];
            assert_eq!(variable_i["name"], format!("i0"));
            assert_eq!(variable_i["exp"], format!("i[0] + func(i)"));
            assert_eq!(variable_i["evaluated"], format!("NotEvaluated"));
            assert_eq!(variable_i["type_indication"], format!("Any"));
            assert_eq!(variable_i["is_array"], false);
        }
    }

    #[test]
    fn simple_declare_variable_4() {
        let src = String::from(r#"
        package test;
        type_null: type = Null;
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            print_tydi_lang_error(&result, src_ptr.clone());
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            let variable_i = &target["package_scope"]["variables"]["type_null"];
            assert_eq!(variable_i["name"], format!("type_null"));
            assert_eq!(variable_i["exp"], format!("Null"));
            assert_eq!(variable_i["evaluated"], format!("NotEvaluated"));
            assert_eq!(variable_i["type_indication"], format!("AnyLogicType"));
            assert_eq!(variable_i["is_array"], false);
        }
    }

    #[test]
    fn simple_declare_variable_5() {
        let src = String::from(r#"
        package test;
        type_null: [type] = {Null, Null, Null};
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            print_tydi_lang_error(&result, src_ptr.clone());
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            let variable_i = &target["package_scope"]["variables"]["type_null"];
            assert_eq!(variable_i["name"], format!("type_null"));
            assert_eq!(variable_i["exp"], format!("{{Null, Null, Null}}"));
            assert_eq!(variable_i["evaluated"], format!("NotEvaluated"));
            assert_eq!(variable_i["type_indication"], format!("AnyLogicType"));
            assert_eq!(variable_i["is_array"], true);
        }
    }

    #[test]
    fn simple_declare_variable_6() {
        let src = String::from(r#"
        package test;
        bit_8 = Bit(8);
        bit_8_type0: Bit(8);
        bit_8_type1: Bit(x);
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            print_tydi_lang_error(&result, src_ptr.clone());
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            let variable_0 = &target["package_scope"]["variables"]["bit_8_type0"];
            assert_eq!(variable_0["name"], format!("bit_8_type0"));
            let variable_1_name: String = variable_0["exp"].as_str().unwrap().to_string();
            let variable_1 = &target["package_scope"]["variables"][&variable_1_name];
            assert_eq!(variable_1["name"], format!("{variable_1_name}"));
        }
        {
            let variable_0 = &target["package_scope"]["variables"]["bit_8_type1"];
            assert_eq!(variable_0["name"], format!("bit_8_type1"));
            let variable_1_name: String = variable_0["exp"].as_str().unwrap().to_string();
            let variable_1 = &target["package_scope"]["variables"][&variable_1_name];
            assert_eq!(variable_1["name"], format!("{variable_1_name}"));
        }
    }

    #[test]
    fn simple_declare_variable_7() {
        let src = String::from(r#"
        package test;
        bit_8 = Bit(8);
        #this is a document#
        Group x <x:[int], y:string> {
            bit_8_type0: Bit(8);
            bit_8_type1: Bit(8);
        }
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            print_tydi_lang_error(&result, src_ptr.clone());
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {

        }
    }


}
