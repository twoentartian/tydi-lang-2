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

mod parse_streamlet;
use parse_streamlet::*;

mod parse_miscellaneous;
use parse_miscellaneous::*;

use crate::error::TydiLangError;
use crate::generate_name::generate_init_value;
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
                return Err(TydiLangError::new(format!("cannot parse the source code, message from parser: {}", message_from_parser), error_location));
            },
            pest::error::ErrorVariant::CustomError { message } => {
                return Err(TydiLangError::new(format!("cannot parse the source code, message: {}", message), CodeLocation::new_unknown()));
            },
        }
    }

    let package_name = generate_init_value();
    let parse_result = parse_result.ok().unwrap();
    let output_package = Package::new(package_name.clone());
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
    {
        let mut output_package_write = output_package.write().unwrap();
        let loc = CodeLocation::new(0, src.len());
        output_package_write.set_code_location(loc);
    }

    return Ok(output_package);
}



#[cfg(test)]
mod test_tydi_lang_src_to_memory_representation {
    use super::*;
    use serde_json::{Value};

    fn get_logic_type<'a>(target: &'a Value, name: & str) -> &'a Value {
        let logic_type_var_name = target[name]["exp"].as_str().unwrap().to_string();
        let output = &target[&logic_type_var_name];
        return output;
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
            println!("{}", result.print(src_ptr.clone()));
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");
        println!("{}", result.read().unwrap().get_code_location().show(src_ptr));
    }

    #[test]
    fn declare_variable_0() {
        let src = String::from(r#"
        package test;
        i = 10;
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            println!("{}", result.print(src_ptr.clone()));
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
    fn declare_variable_1() {
        let src = String::from(r#"
        package test;
        i:int = 10;
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            println!("{}", result.print(src_ptr.clone()));
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
    fn declare_variable_2() {
        let src = String::from(r#"
        package test;
        i:[int] = {10, 20, 30};
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            println!("{}", result.print(src_ptr.clone()));
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
    fn declare_variable_3() {
        let src = String::from(r#"
        package test;
        i:[int] = {10, 20, 30};
        i0 = i[0] + func(i);
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            println!("{}", result.print(src_ptr.clone()));
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
    fn declare_simple_type() {
        let src = String::from(r#"
        package test;
        type_null: type = Null;
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            println!("{}", result.print(src_ptr.clone()));
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
    fn declare_type_array() {
        let src = String::from(r#"
        package test;
        type_null: [type] = {Null, Null, Null};
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            println!("{}", result.print(src_ptr.clone()));
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
    fn declare_bit_0() {
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
            println!("{}", result.print(src_ptr.clone()));
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
            assert_eq!(variable_1["evaluated"], format!("PreEvaluatedLogicType"))
        }
        {
            let variable_0 = &target["package_scope"]["variables"]["bit_8_type1"];
            assert_eq!(variable_0["name"], format!("bit_8_type1"));
            let variable_1_name: String = variable_0["exp"].as_str().unwrap().to_string();
            let variable_1 = &target["package_scope"]["variables"][&variable_1_name];
            assert_eq!(variable_1["name"], format!("{variable_1_name}"));
            assert_eq!(variable_1["evaluated"], format!("PreEvaluatedLogicType"))
        }
    }

    #[test]
    fn declare_group_0() {
        let src = String::from(r#"
        package test;
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
            println!("{}", result.print(src_ptr.clone()));
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            let group_x = &target["package_scope"]["variables"]["x"];
            assert_eq!(group_x["name"], format!("x"));
            let group_x_variable = &group_x["value"][0]["LogicGroupType"];
            assert!(!group_x_variable.is_null());
            let group_x_variable_bit_8_type0 = &group_x_variable["scope"]["variables"]["bit_8_type0"];
            assert_eq!(group_x_variable_bit_8_type0["name"], format!("bit_8_type0"));
        }
    }

    #[test]
    fn declare_union_0() {
        let src = String::from(r#"
        package test;
        bit_8 = Bit(8);
        #this is a document#
        Union x <x:[int], y:string> {
            bit_8_type0: Bit(8)[2];
            bit_8_type1: Bit(8);
        }
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            println!("{}", result.print(src_ptr.clone()));
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            let group_x = &target["package_scope"]["variables"]["x"];
            assert_eq!(group_x["name"], format!("x"));
            let group_x_variable = &group_x["value"][0]["LogicUnionType"];
            assert!(!group_x_variable.is_null());
            let group_x_variable_bit_8_type0 = &group_x_variable["scope"]["variables"]["bit_8_type0"];
            assert_eq!(group_x_variable_bit_8_type0["name"], format!("bit_8_type0"));
            let logic_type = get_logic_type(&group_x_variable["scope"]["variables"], "bit_8_type0");
            assert_eq!(logic_type["is_array"], true);
        }
    }

    #[test]
    fn declare_stream_0() {
        let src = String::from(r#"
        package test;

        bit8 = Bit(8);
        bit8_stream : Stream(Bit(8), d=2, throughput=2.0, s="Sync");
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            println!("{}", result.print(src_ptr.clone()));
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            let bit8_stream = get_logic_type(&target["package_scope"]["variables"], "bit8_stream");
            assert_eq!(bit8_stream["exp"], serde_json::Value::Null);
            assert_eq!(bit8_stream["value"][0]["LogicStreamType"]["dimension"], "2");
        }
    }

    #[test]
    fn declare_package_reference() {
        let src = String::from(r#"
        package test;

        use test1;
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            println!("{}", result.print(src_ptr.clone()));
            return;
        }
        let result = result.ok().unwrap();
        let json_output = serde_json::to_string_pretty(&*result.read().unwrap()).ok().unwrap();
        println!("{json_output}");

        let target: Value = serde_json::from_str(&json_output).unwrap();
        {
            assert_eq!(target["package_scope"]["variables"]["test1"]["type_indication"], "PackageReference");
        }
    }

    #[test]
    fn declare_streamlet() {
        let src = String::from(r#"
        package test;

        # this is a document #
        streamlet x <arg0: int, arg1: [type]> @NoTypeCheck {
            value = 42;

            //port_0: Stream(Bit(8)) in;
        }
        "#);
        let src_ptr = Some(Arc::new(RwLock::new(src.clone())));
        let result = tydi_lang_src_to_memory_representation(src);
        if result.is_err() {
            let result = result.err().unwrap();
            println!("{}", result.print(src_ptr.clone()));
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
