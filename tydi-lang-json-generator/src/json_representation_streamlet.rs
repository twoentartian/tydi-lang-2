use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::Serialize;
use tydi_lang_parser::trait_common::HasDocument;

use crate::json_representation_logic_type::LogicType;
use crate::json_representation_all::JsonRepresentation;
use crate::name_conversion;
use crate::util::{generate_init_name, GetName};
use tydi_lang_parser::tydi_memory_representation::{self, Project, Scope, GlobalIdentifier, GetScope};


#[derive(Clone, Debug, Serialize)]
pub enum PortDirection {
    In,
    Out,
    Unknown,
}

#[derive(Clone, Debug, Serialize)]
pub struct Port {
    #[serde(skip)]
    pub name: String,
    logic_type: LogicType,
    direction: PortDirection,
    document: Option<String>,
}

impl Port {
    pub fn new() -> Self {
        let output = Self {
            name: generate_init_name(),
            logic_type: LogicType::Unknwon,
            direction: PortDirection::Unknown,
            document: None,
        };
        return output;
    }

    pub fn translate_from_tydi_project_port(tydi_project: Arc<RwLock<Project>>, target_port: Arc<RwLock<tydi_memory_representation::Port>>, target_var_scope: Arc<RwLock<Scope>>) -> Result<(Port, JsonRepresentation), String> {
        let mut output_port = Port::new();
        output_port.name = name_conversion::get_global_variable_name_with_parent_scope(target_port.clone());
        let target_port_direction = target_port.read().unwrap().get_direction();
        match target_port_direction {
            tydi_memory_representation::PortDirection::In => {
                output_port.direction = PortDirection::In;
            },
            tydi_memory_representation::PortDirection::Out => {
                output_port.direction = PortDirection::Out;
            },
            tydi_memory_representation::PortDirection::Unknown => unreachable!(),
        }
        let target_port_logic_type = target_port.read().unwrap().get_logical_type();
        let (result_logic_type, mut dependencies) = LogicType::translate_from_tydi_project(tydi_project.clone(), target_port_logic_type.clone())?;
        output_port.logic_type = result_logic_type;
        output_port.document = target_port.read().unwrap().get_document();
        let mut output_json_representation = JsonRepresentation::new();
        output_json_representation.logic_types.append(&mut dependencies);
        return Ok((output_port, output_json_representation));
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Streamlet {
    #[serde(skip)]
    pub name: String,
    ports: BTreeMap<String, Port>,
    document: Option<String>,
}

impl GetName for Streamlet {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl Streamlet {
    pub fn new() -> Self {
        let output = Self {
            name: generate_init_name(),
            ports: BTreeMap::new(),
            document: None,
        };
        return output;
    }

    pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, target_var: Arc<RwLock<tydi_memory_representation::Variable>>) -> Result<(Arc<RwLock<Streamlet>>, JsonRepresentation), String> {
        let target_var_value = target_var.read().unwrap().get_value();

        match &target_var_value {
            tydi_memory_representation::TypedValue::Streamlet(streamlet) => {
                return Self::translate_from_tydi_project_streamlet(tydi_project.clone(), streamlet.clone(), target_var.read().unwrap().get_parent_scope().expect("bug: variable with unknown parent scope"));
            },
            _ => unreachable!()
        }

    }

    pub fn translate_from_tydi_project_streamlet(tydi_project: Arc<RwLock<Project>>, target_streamlet: Arc<RwLock<tydi_memory_representation::Streamlet>>, target_streamlet_scope: Arc<RwLock<Scope>>) -> Result<(Arc<RwLock<Streamlet>>, JsonRepresentation), String> {
        let mut output_dependency = JsonRepresentation::new();
        let mut output_streamlet = Streamlet::new();

        let target_var_name = name_conversion::get_global_variable_name_with_parent_scope(target_streamlet.clone());
        output_streamlet.name = target_var_name.clone();

        let streamlet_scope = target_streamlet.read().unwrap().get_scope();
        let streamlet_all_vars = streamlet_scope.read().unwrap().get_variables();
        for (_, var) in streamlet_all_vars {
            let var_value = var.read().unwrap().get_value();
            match &var_value {
                tydi_memory_representation::TypedValue::Port(port) => {
                    let (port, mut dependencies) = Port::translate_from_tydi_project_port(tydi_project.clone(), port.clone(), streamlet_scope.clone())?;
                    output_dependency.append(&mut dependencies);
                    output_streamlet.ports.insert(port.name.clone(), port);
                },
                _ => (),
            }
        }

        //document
        {
            output_streamlet.document = target_streamlet.read().unwrap().get_document();
        }

        let output_streamlet = Arc::new(RwLock::new(output_streamlet));
        output_dependency.streamlets.insert(target_var_name.clone(), output_streamlet.clone());

        return Ok((output_streamlet, output_dependency));
    }

}