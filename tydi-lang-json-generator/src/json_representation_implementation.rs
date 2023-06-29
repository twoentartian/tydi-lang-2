use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};
use serde::ser::SerializeStruct;

use tydi_lang_parser::tydi_memory_representation::{self, Project, TypedValue};
use tydi_lang_parser::tydi_memory_representation::scope::GetScope;
use tydi_lang_parser::trait_common::GetName;

use crate::json_representation_value::Value;
use crate::name_conversion;

#[derive(Clone, Debug, Serialize)]
pub enum ImplementationType {
    Normal,
    TemplateInstance(String, BTreeMap<usize, Value>),
}

#[derive(Clone, Debug, Serialize)]
pub struct Connection {
    src_port_name: String,
    src_port_owner_name: String,

    sink_port_name: String,
    sink_port_owner_name: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ImplementationInstance {
    derived_implementation: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Implementation {
    connections: BTreeMap<String, Connection>,
    Implementation_instances: BTreeMap<String, ImplementationInstance>,
}