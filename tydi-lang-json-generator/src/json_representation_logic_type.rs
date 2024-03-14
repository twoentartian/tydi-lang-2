use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::Serialize;
use serde::ser::SerializeStruct;

use tydi_lang_parser::tydi_memory_representation::{self, Project, TypedValue};
use tydi_lang_parser::tydi_memory_representation::scope::GetScope;
use tydi_lang_parser::trait_common::{GetName, HasDocument};

use crate::name_conversion::{self, get_global_variable_name_with_parent_scope};

#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum LogicType {
    Unknwon,
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
                state.serialize_field("value", v)?;
            },
        };
        
        state.end()
    }

}

impl LogicType {
    pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, target_var: Arc<RwLock<tydi_memory_representation::Variable>>) -> Result<(LogicType, BTreeMap<String, Arc<RwLock<LogicType>>>), String> {
        let target_var_name = name_conversion::get_global_variable_name(target_var.clone());
        let var_value = target_var.read().unwrap().get_value();

        return Self::translate_from_tydi_project_type_value(tydi_project.clone(), &var_value, target_var_name);
    }

    pub fn translate_from_tydi_project_type_value(tydi_project: Arc<RwLock<Project>>, target_type_value: &tydi_memory_representation::TypedValue, default_var_name: String) -> Result<(LogicType, BTreeMap<String, Arc<RwLock<LogicType>>>), String> {
        let mut output_dependency = BTreeMap::new();
        let output_type;
        let mut target_var_name = default_var_name;
        
        match &target_type_value {
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
                        //we don't update the target_var_name because for logic bit.
                    },
                    tydi_memory_representation::LogicType::LogicGroupType(v) => {
                        let results = LogicGroup::translate_from_tydi_project(tydi_project.clone(), v.clone())?;
                        let (logic_group, mut dependencies) = results;
                        output_dependency.append(&mut dependencies);
                        output_type = LogicType::Group(Arc::new(RwLock::new(logic_group)));
                        target_var_name = get_global_variable_name_with_parent_scope(v.clone());
                    },
                    tydi_memory_representation::LogicType::LogicUnionType(v) => {
                        let results = LogicUnion::translate_from_tydi_project(tydi_project.clone(), v.clone())?;
                        let (logic_union, mut dependencies) = results;
                        output_dependency.append(&mut dependencies);
                        output_type = LogicType::Union(Arc::new(RwLock::new(logic_union)));
                        target_var_name = get_global_variable_name_with_parent_scope(v.clone());
                    },
                    tydi_memory_representation::LogicType::LogicStreamType(v) => {
                        let results = LogicStream::translate_from_tydi_project(tydi_project.clone(), v.clone())?;
                        let (logic_stream, mut dependencies) = results;
                        output_dependency.append(&mut dependencies);
                        output_type = LogicType::Stream(Arc::new(RwLock::new(logic_stream)));
                        target_var_name = get_global_variable_name_with_parent_scope(v.clone());
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
            _ => unreachable!("{} is not a logic type", target_type_value.get_brief_info()),
        }

        //we should always return a reference to it
        output_dependency.insert(target_var_name.clone(), Arc::new(RwLock::new(output_type)));

        return Ok((LogicType::Ref(target_var_name), output_dependency));
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
                output_group.elements.insert(var_name.clone(), logic_type);
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
                output_group.elements.insert(var_name.clone(), logic_type);
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
            let stream_type = match stream_type_value.try_get_referenced_variable() {
                Some(var) => var,
                None => stream_type,
            };

            let result = LogicType::translate_from_tydi_project(tydi_project.clone(), stream_type.clone());
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let (stream_type, mut dependencies) = result.ok().unwrap();
            output_dependency.append(&mut dependencies);
            match stream_type {
                LogicType::Ref(r) => {
                    output_stream.stream_type = LogicType::Ref(r);
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