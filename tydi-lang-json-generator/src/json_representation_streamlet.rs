use std::collections::BTreeMap;

use serde::{Serialize};

use crate::json_representation_logic_type::LogicType;




#[derive(Clone, Debug, Serialize)]
pub enum PortDirection {
    In,
    Out,
}

#[derive(Clone, Debug, Serialize)]
pub struct Port {
    name: String,
    logic_type: LogicType,
    direction: PortDirection
}

#[derive(Clone, Debug, Serialize)]
pub struct Streamlet {
    ports: BTreeMap<String, Port>,
}