use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};
use serde::ser::SerializeStruct;

use tydi_lang_parser::tydi_memory_representation::{self, Project, TypedValue};
use tydi_lang_parser::tydi_memory_representation::scope::GetScope;
use tydi_lang_parser::trait_common::GetName;

use crate::name_conversion;

#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum LogicType {
    Null,
    Bit(usize),
    Group(Arc<RwLock<LogicGroup>>),
    Union(Arc<RwLock<LogicUnion>>),
    Stream(Arc<RwLock<LogicStream>>),
    Ref(String),
}

impl serde::Serialize for LogicType {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> 
    where S: serde::Serializer {
        let mut state = serializer.serialize_struct("logic_type", 2)?;
        let enum_type_str: &'static str = self.into();
        state.serialize_field("type", enum_type_str)?;
        match &self {
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
                state.serialize_field("value", v)?;
            },
        };
        
        state.end()
    }

}

impl LogicType {
    pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, target_var: Arc<RwLock<tydi_memory_representation::Variable>>) -> Result<(LogicType, BTreeMap<String, Arc<RwLock<LogicType>>>), String> {
        let mut output_dependency = BTreeMap::new();
        let output_type;
        
        let target_var_name = name_conversion::get_global_variable_name(target_var.clone());
        let var_value = target_var.read().unwrap().get_value();
        match &var_value {
            TypedValue::LogicTypeValue(logical_type) => {
                let logical_type = logical_type.read().unwrap().clone();
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
                    },
                    tydi_memory_representation::LogicType::LogicGroupType(v) => {
                        let mut results = LogicGroup::translate_from_tydi_project(tydi_project.clone(), v.clone());
                        if results.is_err() {
                            return Err(results.err().unwrap());
                        }
                        let (logic_group, mut dependencies) = results.ok().unwrap();
                        output_dependency.append(&mut dependencies);
                        output_type = LogicType::Group(Arc::new(RwLock::new(logic_group)));
                    },
                    tydi_memory_representation::LogicType::LogicUnionType(v) => {
                        todo!()
                    },
                    tydi_memory_representation::LogicType::LogicStreamType(v) => {
                        let results = LogicStream::translate_from_tydi_project(tydi_project.clone(), v.clone());
                        if results.is_err() {
                            return Err(results.err().unwrap());
                        }
                        let (logic_stream, mut dependencies) = results.ok().unwrap();
                        output_dependency.append(&mut dependencies);
                        output_type = LogicType::Stream(Arc::new(RwLock::new(logic_stream)));
                    },
                }

            },
            TypedValue::RefToVar(ref_var) => {
                let results = LogicType::translate_from_tydi_project(tydi_project.clone(), ref_var.clone());
                if results.is_err() {
                    return Err(results.err().unwrap());
                }
                let (logic_ref, mut dependencies) = results.ok().unwrap();
                output_dependency.append(&mut dependencies);
                // output_type = LogicType::Ref(name_conversion::get_global_variable_name(ref_var.clone()));
                output_type = logic_ref;
            },
            _ => unreachable!("{} is not a logic type", var_value.get_brief_info()),
        }


        //we should always return a reference to it
        let target_var_name = name_conversion::get_global_variable_name(target_var.clone());
        output_dependency.insert(target_var_name.clone(), Arc::new(RwLock::new(output_type)));

        return Ok((LogicType::Ref(target_var_name), output_dependency));
    }

}

#[derive(Clone, Debug, Serialize)]
pub struct LogicGroup {
    elements: BTreeMap<String, LogicType>,
}

impl LogicGroup {
    pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, tydi_target: Arc<RwLock<tydi_memory_representation::LogicGroup>>) -> Result<(LogicGroup, BTreeMap<String, Arc<RwLock<LogicType>>>), String> {
        let mut output_dependency = BTreeMap::new();
        let mut output_group = LogicGroup {
            elements: BTreeMap::new(),
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
                output_group.elements.insert(var_name.clone(), logic_type);
            }
        }

        return Ok((output_group, output_dependency));
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct LogicUnion {
    elements: BTreeMap<String, String>,
}



#[derive(Clone, Debug, Serialize)]
pub struct LogicStream {
    stream_type: String,
    dimension: i128,
    user_type: String,
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
            stream_type: format!("???"),
            dimension: 0,
            user_type: format!("???"),
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
            let stream_type_var = stream_type.read().unwrap().get_value().try_get_referenced_variable().expect("bug: the stream type should be a reference");

            let result = LogicType::translate_from_tydi_project(tydi_project.clone(), stream_type_var.clone());
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let (stream_type, mut dependencies) = result.ok().unwrap();
            output_dependency.append(&mut dependencies);
            match stream_type {
                LogicType::Ref(r) => {
                    output_stream.stream_type = r;
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
            //user type should be a reference
            let user_type = tydi_target.read().unwrap().get_user_type();
            let user_type_var = user_type.read().unwrap().get_value().try_get_referenced_variable().expect("bug: the stream type should be a reference");

            let result = LogicType::translate_from_tydi_project(tydi_project.clone(), user_type_var.clone());
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let (user_type, mut dependencies) = result.ok().unwrap();
            output_dependency.append(&mut dependencies);
            match user_type {
                LogicType::Ref(r) => {
                    output_stream.user_type = r;
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