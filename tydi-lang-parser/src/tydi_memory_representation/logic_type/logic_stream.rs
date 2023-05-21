use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::{generate_access_pub, generate_get_pub};
use crate::tydi_memory_representation::{TypeIndication, CodeLocation, TraitCodeLocationAccess, Variable, TypedValue, LogicType};

use crate::trait_common::{GetName, AccessProperty};

pub enum LogicStreamProperty {
    #[allow(non_camel_case_types)]
    dimension,
    #[allow(non_camel_case_types)]
    user_type,
    #[allow(non_camel_case_types)]
    throughput,
    #[allow(non_camel_case_types)]
    synchronicity,
    #[allow(non_camel_case_types)]
    complexity,
    #[allow(non_camel_case_types)]
    direction,
    #[allow(non_camel_case_types)]
    keep,
}

impl std::convert::TryFrom<String> for LogicStreamProperty {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "dimension" || value == "d" {
            return Ok(LogicStreamProperty::dimension);
        }
        if value == "user_type" || value == "u" {
            return Ok(LogicStreamProperty::user_type);
        }
        if value == "throughput" || value == "t" {
            return Ok(LogicStreamProperty::throughput);
        }
        if value == "synchronicity" || value == "s" {
            return Ok(LogicStreamProperty::synchronicity);
        }
        if value == "complexity" || value == "c" {
            return Ok(LogicStreamProperty::complexity);
        }
        if value == "direction" || value == "r" {
            return Ok(LogicStreamProperty::direction);
        }
        if value == "keep" || value == "x" {
            return Ok(LogicStreamProperty::keep);
        }
        return Err(());
    }
}

impl LogicStreamProperty {
    pub fn get_stream_property_var(&self, logic_stream: &LogicStream) -> Arc<RwLock<Variable>> {
        match self {
            LogicStreamProperty::dimension => return logic_stream.dimension.clone(),
            LogicStreamProperty::user_type => return logic_stream.user_type.clone(),
            LogicStreamProperty::throughput => return logic_stream.throughput.clone(),
            LogicStreamProperty::synchronicity => return logic_stream.synchronicity.clone(),
            LogicStreamProperty::complexity => return logic_stream.complexity.clone(),
            LogicStreamProperty::direction => return logic_stream.direction.clone(),
            LogicStreamProperty::keep => return logic_stream.keep.clone(),
        }
    }

    pub fn set_stream_property_var(&self, logic_stream: &mut LogicStream, var: Arc<RwLock<Variable>>) {
        match self {
            LogicStreamProperty::dimension => logic_stream.dimension = var.clone(),
            LogicStreamProperty::user_type => logic_stream.user_type = var.clone(),
            LogicStreamProperty::throughput => logic_stream.throughput = var.clone(),
            LogicStreamProperty::synchronicity => logic_stream.synchronicity = var.clone(),
            LogicStreamProperty::complexity => logic_stream.complexity = var.clone(),
            LogicStreamProperty::direction => logic_stream.direction = var.clone(),
            LogicStreamProperty::keep => logic_stream.keep = var.clone(),
        }
    }

    pub fn get_full_name(&self) -> &str {
        match self {
            LogicStreamProperty::dimension => return "dimension",
            LogicStreamProperty::user_type => return "user_type",
            LogicStreamProperty::throughput => return "throughput",
            LogicStreamProperty::synchronicity => return "synchronicity",
            LogicStreamProperty::complexity => return "complexity",
            LogicStreamProperty::direction => return "direction",
            LogicStreamProperty::keep => return "keep",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct LogicStream {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    stream_type: Arc<RwLock<Variable>>,

    location_define: CodeLocation,

    // stream properties
    /// d, default: 1. Candidate: int > 0
    #[serde(with = "crate::serde_serialization::serialize_variable_value_only")]
    dimension: Arc<RwLock<Variable>>,
    /// u, default: Null. Candidate: Null, Bit, Group, Union or a composite of them (including an type alias)
    #[serde(with = "crate::serde_serialization::serialize_variable_value_only")]
    user_type: Arc<RwLock<Variable>>,
    /// t, default: 1.0. Candidate: float
    #[serde(with = "crate::serde_serialization::serialize_variable_value_only")]
    throughput: Arc<RwLock<Variable>>,
    /// s, default: Sync. Candidate: "Sync", "Flatten", "Desync" and "FlatDesync"
    #[serde(with = "crate::serde_serialization::serialize_variable_value_only")]
    synchronicity: Arc<RwLock<Variable>>,
    /// c, default: 1. Candidate: int 1~7
    #[serde(with = "crate::serde_serialization::serialize_variable_value_only")]
    complexity: Arc<RwLock<Variable>>,
    /// r, default: Forward. Candidate: Forward, Reverse
    #[serde(with = "crate::serde_serialization::serialize_variable_value_only")]
    direction: Arc<RwLock<Variable>>,
    /// x, default: false. Candidate: true, false
    #[serde(with = "crate::serde_serialization::serialize_variable_value_only")]
    keep: Arc<RwLock<Variable>>,
}

impl GetName for LogicStream {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl TraitCodeLocationAccess for LogicStream {
    fn set_code_location(& mut self, loc: CodeLocation) {
        self.location_define = loc;
    }

    fn get_code_location(&self) -> CodeLocation {
        return self.location_define.clone();
    }
}

impl AccessProperty for LogicStream {
    fn access_porperty(&self, property_name: String) -> Option<Arc<RwLock<Variable>>> {
        let property_type_result = LogicStreamProperty::try_from(property_name);
        if property_type_result.is_err() {
            return None;
        }
        let property_type_result = property_type_result.ok().unwrap();
        let property_var = property_type_result.get_stream_property_var(self);
        return Some(property_var);
    }
}

impl LogicStream {
    pub fn new(name: String, stream_type: Option<String>) -> Arc<RwLock<Self>> {
        let name: &String = &name;
        let stream_type_var = Variable::new(format!("{}_{name}", "stream_type"), stream_type);
        {
            let mut stream_type_var_write = stream_type_var.write().unwrap();
            stream_type_var_write.set_type_indication(TypeIndication::AnyLogicType);
        }
        let output = Self {
            name: name.clone(),
            stream_type: stream_type_var,
            location_define: CodeLocation::new_unknown(),
            dimension: Variable::new_predefined(format!("{}_{name}", "dimension"), TypedValue::IntValue(1)),
            user_type: Variable::new_predefined(format!("{}_{name}", "user_type"), TypedValue::LogicTypeValue(Arc::new(RwLock::new(LogicType::LogicNullType)))),
            throughput: Variable::new_predefined(format!("{}_{name}", "throughput"), TypedValue::FloatValue(1.0)),
            synchronicity: Variable::new_predefined(format!("{}_{name}", "synchronicity"), TypedValue::StringValue(format!("Sync"))),
            complexity: Variable::new_predefined(format!("{}_{name}", "dimension"), TypedValue::IntValue(1)),
            direction: Variable::new_predefined(format!("{}_{name}", "synchronicity"), TypedValue::StringValue(format!("Forward"))),
            keep: Variable::new_predefined(format!("{}_{name}", "synchronicity"), TypedValue::BoolValue(false)),
        };

        return Arc::new(RwLock::new(output));
    }

    pub fn apply_property(&mut self, property_type: LogicStreamProperty, property: Arc<RwLock<Variable>>) {
        property_type.set_stream_property_var(self, property);
    }

    pub fn apply_property_var(&mut self, property: Arc<RwLock<Variable>>) -> Result<(),String> {
        let property_name = property.read().unwrap().get_name();
        let property_type = LogicStreamProperty::try_from(property_name.clone());
        if property_type.is_err() {
            return Err(format!("unknown property: {}", property_name));
        }
        let property_type = property_type.ok().unwrap();
        self.apply_property(property_type, property.clone());
        return Ok(());
    }

    generate_get_pub!(stream_type, Arc<RwLock<Variable>>, get_stream_type);
    generate_get_pub!(dimension, Arc<RwLock<Variable>>, get_dimension);
    generate_get_pub!(user_type, Arc<RwLock<Variable>>, get_user_type);
    generate_get_pub!(throughput, Arc<RwLock<Variable>>, get_throughput);
    generate_get_pub!(synchronicity, Arc<RwLock<Variable>>, get_synchronicity);
    generate_get_pub!(complexity, Arc<RwLock<Variable>>, get_complexity);
    generate_get_pub!(direction, Arc<RwLock<Variable>>, get_direction);
    generate_get_pub!(keep, Arc<RwLock<Variable>>, get_keep);
}
