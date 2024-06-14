use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;
use std::vec;

use serde::Serialize;
use serde::ser::SerializeStruct;

use tydi_lang_parser::tydi_memory_representation::Variable;
use tydi_lang_parser::tydi_memory_representation::{self, code_location::TraitCodeLocationAccess, Project, TypedValue, scope::GetScope, GlobalIdentifier};
use tydi_lang_parser::trait_common::{GetName, HasDocument};

use crate::name_conversion::{self, get_global_variable_name_with_parent_scope};
use crate::util::generate_init_name;

#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum LogicType {
    Unknwon,
    Null,
    Bit(usize),
    Group(Arc<RwLock<LogicGroup>>),
    Union(Arc<RwLock<LogicUnion>>),
    Stream(Arc<RwLock<LogicStream>>),
    Ref(RefInfo),
}

#[derive(Clone, Debug)]
pub struct RefInfo {
    ref_name: String,
    info: Option<Info>,
    alias: Vec<String>,
}

impl RefInfo {
    pub fn new(ref_name: String) -> Self {
        return Self {
            ref_name: ref_name,
            info: None,
            alias: vec![],
        };
    }

    pub fn set_info_raw(&mut self, name: String, scope_name: Option<String>, src_file: Option<String>, loc_begin: Option<usize>, loc_end: Option<usize>) {
        self.info = Some(Info::new(name, scope_name, src_file, loc_begin, loc_end));
    }

    pub fn set_info_from_var(&mut self, var: Arc<RwLock<Variable>>) {
        let target_var_parent_scope = var.read().unwrap().get_parent_scope();
        let target_var_parent_scope_name = match target_var_parent_scope {
            Some(v) => {
                Some(v.read().unwrap().get_name())
            },
            None => None
        };
        let target_var_loc = var.read().unwrap().get_code_location();
        let loc_begin = target_var_loc.begin.clone();
        let loc_end = target_var_loc.end.clone();
        let raw_name = var.read().unwrap().get_name();
        let src_file = target_var_loc.src_file.file_name.clone();
        self.set_info_raw(raw_name, target_var_parent_scope_name, Some(src_file), loc_begin, loc_end);

        let alias = var.read().unwrap().get_alias();
        for a in alias {
            self.add_alias(a);
        }
    }

    pub fn set_info(&mut self, info: Option<Info>) {
        self.info = info.clone();
    }

    pub fn add_alias(&mut self, alias: String) {
        self.alias.push(alias);
    }
}

#[derive(Clone, Debug)]
pub struct Info {
    name: String,
    declared_in_scope: Option<String>,
    src_file: Option<String>,
    location_begin: Option<usize>,
    location_end: Option<usize>,
}

impl serde::Serialize for Info {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> 
    where S: serde::Serializer {
        let mut state = serializer.serialize_struct("Info", 4)?;
        
        state.serialize_field("name", &self.name)?;
        if self.declared_in_scope.is_some() {
            state.serialize_field("declared_in_scope", &self.declared_in_scope)?;
        }
        let loc_begin = 
        if self.location_begin.is_some() { self.location_begin.unwrap().to_string() }
        else { String::from("?") };
        let loc_end = 
        if self.location_end.is_some() { self.location_end.unwrap().to_string() }
        else { String::from("?") };
        let src_file_path = 
        if self.src_file.is_some() { self.src_file.clone().unwrap() }
        else { String::from("?") };

        state.serialize_field("src_location", &format!("{src_file_path}:{loc_begin}~{loc_end}"))?;

        state.end()
    }

}

impl Info {
    pub fn new(name: String, scope_name: Option<String>, src_file: Option<String>, loc_begin: Option<usize>, loc_end: Option<usize>) -> Self {
        return Self {
            name: name,
            declared_in_scope: scope_name,
            src_file: src_file,
            location_begin: loc_begin,
            location_end: loc_end,
        }
    }

    pub fn new_init() -> Self {
        return Self {
            name: generate_init_name(),
            declared_in_scope: None,
            src_file: None,
            location_begin: None,
            location_end: None,
        }
    }
}

impl serde::Serialize for LogicType {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> 
    where S: serde::Serializer {
        let mut state = serializer.serialize_struct("logic_type", 2)?;
        let enum_type_str: &'static str = self.into();
        state.serialize_field("type", enum_type_str)?;
        match &self {
            LogicType::Unknwon => unreachable!("unknown logic type"),
            LogicType::Null => {
                // state.serialize_field("value", LogicNullExp)?;   //do nothing because type already says this is Null
            },
            LogicType::Bit(v) => {
                state.serialize_field("value", &v)?;
            },
            LogicType::Group(v) => {
                state.serialize_field("value", &*v.read().unwrap())?;
            },
            LogicType::Union(v) => {
                state.serialize_field("value", &*v.read().unwrap())?;
            },
            LogicType::Stream(v) => {
                state.serialize_field("value", &*v.read().unwrap())?;
            },
            LogicType::Ref(v) => {
                state.serialize_field("value", &v.ref_name)?;
                state.serialize_field("alias", &v.alias)?;
                state.serialize_field("info", &v.info)?;
            },
        };
        
        state.end()
    }

}

impl LogicType {
    pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, target_var: Arc<RwLock<tydi_memory_representation::Variable>>) -> Result<(Vec<LogicType>, BTreeMap<String, Arc<RwLock<LogicType>>>), String> {
        let target_var_name = name_conversion::get_global_variable_name(target_var.clone());
        let var_value = target_var.read().unwrap().get_value();

        return Self::translate_from_tydi_project_type_value(tydi_project.clone(), &var_value, target_var_name, Some(target_var.clone()));
    }

    pub fn translate_from_tydi_project_type_value(tydi_project: Arc<RwLock<Project>>, target_type_value: &tydi_memory_representation::TypedValue, default_var_name: String, raw_var: Option<Arc<RwLock<tydi_memory_representation::Variable>>>) -> Result<(Vec<LogicType>, BTreeMap<String, Arc<RwLock<LogicType>>>), String> {
        let mut output_dependency = BTreeMap::new();
        let mut target_var_name = default_var_name;

        let mut output_types = vec![];

        match &target_type_value {
            TypedValue::LogicTypeValue(logical_type) => {
                let (output_type, mut dependencies, info) = Self::translate_single_basic_logic_type(tydi_project.clone(), logical_type.clone(), &mut target_var_name)?;
                output_dependency.append(&mut dependencies);
                output_dependency.insert(target_var_name.clone(), Arc::new(RwLock::new(output_type.clone())));
                // output_types.push(output_type.clone());
                let mut ref_info = RefInfo::new(target_var_name.clone());
                ref_info.set_info(info);
                output_types.push(LogicType::Ref(ref_info));
            },
            TypedValue::RefToVar(ref_var) => {
                let results = LogicType::translate_from_tydi_project(tydi_project.clone(), ref_var.clone());
                if results.is_err() {
                    return Err(results.err().unwrap());
                }
                let (mut output_type, mut dependencies) = results.ok().unwrap();
                output_dependency.append(&mut dependencies);

                assert!(output_type.len() > 0);

                {
                    for single_type in &mut output_type {
                        match single_type {
                            LogicType::Ref(ref_target) => {
                                match raw_var.clone() {
                                    Some(raw_var) => {
                                        ref_target.set_info_from_var(raw_var.clone());
                                    },
                                    None => (),
                                };
                            },
                            _ => (),
                        }
                    }
                }

                output_types.append(&mut output_type);
            },
            TypedValue::Array(array) => {
                for (index, single_value) in array.iter().enumerate() {
                    match single_value {
                        TypedValue::LogicTypeValue(single_logic_type) => {
                            let (output_type, mut dependencies, info) = Self::translate_single_basic_logic_type(tydi_project.clone(), single_logic_type.clone(), &mut target_var_name)?;
                            output_dependency.append(&mut dependencies);
                            let element_var_name = format!("{}_for{}", &target_var_name, index);
                            output_dependency.insert(element_var_name.clone(), Arc::new(RwLock::new(output_type.clone())));
                            let mut ref_info = RefInfo::new(element_var_name);
                            
                            ref_info.set_info(info);
                            output_types.push(LogicType::Ref(ref_info));
                        },
                        _ => (),
                    }
                }
            },
            _ => unreachable!("{} is not a logic type", target_type_value.get_brief_info()),
        }

        return Ok((output_types, output_dependency));
    }

    fn translate_single_basic_logic_type(tydi_project: Arc<RwLock<Project>>, target_type: Arc<RwLock<tydi_memory_representation::LogicType>>, target_var_name: &mut String) -> Result<(LogicType, BTreeMap<String, Arc<RwLock<LogicType>>>, Option<Info>), String> {
        let mut output_dependency = BTreeMap::new();
        let output_type;
        let output_info: Option<Info> = None;

        let logical_type = target_type.read().unwrap().clone();
        match logical_type {
            tydi_memory_representation::LogicType::LogicNullType => {
                output_type = LogicType::Null;
            },
            tydi_memory_representation::LogicType::LogicBitType(logic_bit) => {
                let bit_width_var = logic_bit.read().unwrap().get_bit_width();
                let bit_width = match bit_width_var.read().unwrap().get_value() {
                    TypedValue::IntValue(i) => i,
                    _ => unreachable!("error in project evaluation: bit width of {} is not an integer", logic_bit.read().unwrap().get_name()),
                };
                output_type = LogicType::Bit(bit_width as usize);
                output_dependency.insert(target_var_name.clone(), Arc::new(RwLock::new(output_type.clone())));
                //we don't update the target_var_name because we set alias
            },
            tydi_memory_representation::LogicType::LogicGroupType(v) => {
                let results = LogicGroup::translate_from_tydi_project(tydi_project.clone(), v.clone())?;
                let (logic_group, mut dependencies) = results;
                output_dependency.append(&mut dependencies);
                output_type = LogicType::Group(Arc::new(RwLock::new(logic_group)));
                *target_var_name = get_global_variable_name_with_parent_scope(v.clone());
            },
            tydi_memory_representation::LogicType::LogicUnionType(v) => {
                let results = LogicUnion::translate_from_tydi_project(tydi_project.clone(), v.clone())?;
                let (logic_union, mut dependencies) = results;
                output_dependency.append(&mut dependencies);
                output_type = LogicType::Union(Arc::new(RwLock::new(logic_union)));
                *target_var_name = get_global_variable_name_with_parent_scope(v.clone());
            },
            tydi_memory_representation::LogicType::LogicStreamType(v) => {
                let results = LogicStream::translate_from_tydi_project(tydi_project.clone(), v.clone())?;
                let (logic_stream, mut dependencies) = results;
                output_dependency.append(&mut dependencies);
                output_type = LogicType::Stream(Arc::new(RwLock::new(logic_stream)));
                //we don't update the target_var_name because we set alias
            },
        }
        return Ok((output_type, output_dependency, output_info));
    }


}

#[derive(Clone, Debug, Serialize)]
pub struct LogicGroup {
    elements: BTreeMap<String, LogicType>,
    document: Option<String>,
}

impl LogicGroup {
    pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, tydi_target: Arc<RwLock<tydi_memory_representation::LogicGroup>>) -> Result<(LogicGroup, BTreeMap<String, Arc<RwLock<LogicType>>>), String> {
        let mut output_dependency = BTreeMap::new();
        let mut output_group = LogicGroup {
            elements: BTreeMap::new(),
            document: tydi_target.read().unwrap().get_document(),
        };
        let scope = tydi_target.read().unwrap().get_scope();
        let variables = scope.read().unwrap().get_variables();
        for (var_name, var) in &variables {
            let is_property = var.read().unwrap().get_is_property_of_scope();
            if is_property {
                let result = LogicType::translate_from_tydi_project(tydi_project.clone(), var.clone());
                if result.is_err() {
                    return Err(result.err().unwrap());
                }
                let (logic_type, mut dependencies) = result.ok().unwrap();
                output_dependency.append(&mut dependencies);

                assert!(logic_type.len() > 0);
                if logic_type.len() == 1 {
                    output_group.elements.insert(var_name.clone(), logic_type[0].clone());
                }
                else {
                    for (index, i) in logic_type.iter().enumerate() {
                        output_group.elements.insert(format!("{}_for{}", &var_name, index), logic_type[index].clone());
                    }
                }
            }
        }

        return Ok((output_group, output_dependency));
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct LogicUnion {    
    elements: BTreeMap<String, LogicType>,
    document: Option<String>,
}

impl LogicUnion {
    pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, tydi_target: Arc<RwLock<tydi_memory_representation::LogicUnion>>) -> Result<(LogicUnion, BTreeMap<String, Arc<RwLock<LogicType>>>), String> {
        let mut output_dependency = BTreeMap::new();
        let mut output_group = LogicUnion {
            elements: BTreeMap::new(),
            document: tydi_target.read().unwrap().get_document(),
        };
        let scope = tydi_target.read().unwrap().get_scope();
        let variables = scope.read().unwrap().get_variables();
        for (var_name, var) in &variables {
            let is_property = var.read().unwrap().get_is_property_of_scope();
            if is_property {
                let result = LogicType::translate_from_tydi_project(tydi_project.clone(), var.clone());
                if result.is_err() {
                    return Err(result.err().unwrap());
                }
                let (logic_type, mut dependencies) = result.ok().unwrap();
                output_dependency.append(&mut dependencies);

                assert!(logic_type.len() > 0);
                if logic_type.len() == 1 {
                    output_group.elements.insert(var_name.clone(), logic_type[0].clone());
                }
                else {
                    for (index, i) in logic_type.iter().enumerate() {
                        output_group.elements.insert(format!("{}_for{}", &var_name, index), logic_type[index].clone());
                    }
                }
            }
        }

        return Ok((output_group, output_dependency));
    }
}



#[derive(Clone, Debug, Serialize)]
pub struct LogicStream {
    stream_type: LogicType,
    dimension: i128,
    user_type: LogicType,
    throughput: f64,
    synchronicity: String,
    complexity: i128,
    direction: String,
    keep: bool,
}

impl LogicStream {
    pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, tydi_target: Arc<RwLock<tydi_memory_representation::LogicStream>>) -> Result<(LogicStream, BTreeMap<String, Arc<RwLock<LogicType>>>), String> {
        let mut output_dependency = BTreeMap::new();

        let mut output_stream = Self {
            stream_type: LogicType::Unknwon,
            dimension: 0,
            user_type: LogicType::Unknwon,
            throughput: 0.0,
            synchronicity: format!("???"),
            complexity: 0,
            direction: format!("???"),
            keep: false,
        };

        //stream type
        {
            //stream type should be a reference
            let stream_type = tydi_target.read().unwrap().get_stream_type();
            let stream_type_value = stream_type.read().unwrap().get_value();
            let stream_type_var = match stream_type_value.try_get_referenced_variable() {
                Some(var) => var,
                None => stream_type,
            };

            let result = LogicType::translate_from_tydi_project(tydi_project.clone(), stream_type_var.clone());
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let (stream_type, mut dependencies) = result.ok().unwrap();
            output_dependency.append(&mut dependencies);
            assert!(stream_type.len() == 1, "the type of a stream should be a single logic type, not an array");
            let mut stream_type = stream_type[0].clone();
            match &mut stream_type {
                LogicType::Ref(r) => {
                    let all_alias = stream_type_var.read().unwrap().get_alias();
                    for a in all_alias {
                        r.add_alias(a);
                    }
                    output_stream.stream_type = LogicType::Ref(r.to_owned());
                },
                _ => unreachable!("should be unreachable")
            }
        }

        //dimension
        {
            let dimension = tydi_target.read().unwrap().get_dimension();
            let dimension_value = dimension.read().unwrap().get_value();
            match dimension_value {
                TypedValue::IntValue(v) => {
                    output_stream.dimension = v;
                },
                _ => unreachable!()
            }
        }

        //user type
        {
            //user type should be a reference or None
            let user_type = tydi_target.read().unwrap().get_user_type();
            let user_type_value = user_type.read().unwrap().get_value();
            let user_type = match user_type_value.try_get_referenced_variable() {
                Some(var) => var,
                None => user_type,
            };

            let result = LogicType::translate_from_tydi_project(tydi_project.clone(), user_type.clone());
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let (user_type, mut dependencies) = result.ok().unwrap();
            output_dependency.append(&mut dependencies);
            assert!(user_type.len() == 1, "the user type of a stream should be a single logic type, not an array");
            let user_type = user_type[0].clone();
            match user_type {
                LogicType::Ref(r) => {
                    output_stream.user_type = LogicType::Ref(r);
                },
                _ => unreachable!("should be unreachable")
            }
        }

        //throughput
        {
            let throughput = tydi_target.read().unwrap().get_throughput();
            let throughput_value = throughput.read().unwrap().get_value();
            match throughput_value {
                TypedValue::FloatValue(v) => {
                    output_stream.throughput = v;
                },
                _ => unreachable!()
            }
        }

        //synchronicity
        {
            let synchronicity = tydi_target.read().unwrap().get_synchronicity();
            let synchronicity_value = synchronicity.read().unwrap().get_value();
            match synchronicity_value {
                TypedValue::StringValue(v) => {
                    output_stream.synchronicity = v;
                },
                _ => unreachable!()
            }
        }

        //complexity
        {
            let complexity = tydi_target.read().unwrap().get_complexity();
            let complexity_value = complexity.read().unwrap().get_value();
            match complexity_value {
                TypedValue::IntValue(v) => {
                    output_stream.complexity = v;
                },
                _ => unreachable!()
            }
        }

        //direction
        {
            let direction = tydi_target.read().unwrap().get_direction();
            let direction_value = direction.read().unwrap().get_value();
            match direction_value {
                TypedValue::StringValue(v) => {
                    output_stream.direction = v;
                },
                _ => unreachable!()
            }
        }

        //keep
        {
            let keep = tydi_target.read().unwrap().get_keep();
            let keep_value = keep.read().unwrap().get_value();
            match keep_value {
                TypedValue::BoolValue(v) => {
                    output_stream.keep = v;
                },
                _ => unreachable!()
            }
        }

        let output_stream_type = LogicType::Stream(Arc::new(RwLock::new(output_stream.clone())));
        output_dependency.insert(tydi_target.read().unwrap().get_name(), Arc::new(RwLock::new(output_stream_type)));

        return Ok((output_stream, output_dependency));
    }
}