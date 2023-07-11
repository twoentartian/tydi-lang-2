use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};

use tydi_lang_parser::trait_common::{GetName, HasDocument};
use tydi_lang_parser::tydi_memory_representation::{self, Project, TypedValue, Scope, GetScope, PortOwner, GlobalIdentifier};

use crate::json_representation_all::{JsonRepresentation};
use crate::json_representation_value::Value;
use crate::json_representation_streamlet::Streamlet;
use crate::name_conversion::{self, get_global_variable_name_with_parent_scope, get_global_variable_name_with_scope};
use crate::util::generate_init_name;

#[derive(Clone, Debug, Serialize)]
pub enum ImplementationType {
    Normal,
    TemplateInstance(String, BTreeMap<usize, Value>),
    Unknown,
}

#[derive(Clone, Debug, Serialize)]
pub struct Net {
    #[serde(skip)]
    name: String,

    src_port_name: String,
    src_port_owner_name: String,

    sink_port_name: String,
    sink_port_owner_name: String,

    document: Option<String>,
}

impl Net {
    pub fn new() -> Self {
        let output = Self {
            name: generate_init_name(),
            src_port_name: generate_init_name(),
            src_port_owner_name: generate_init_name(),
            sink_port_name: generate_init_name(),
            sink_port_owner_name: generate_init_name(),
            document: None,
        };
        return output;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ImplementationInstance {
    #[serde(skip)]
    name: String,
    derived_implementation: String,
    document: Option<String>,
}

impl ImplementationInstance {
    pub fn new() -> Self {
        let output = Self {
            name: generate_init_name(),
            derived_implementation: generate_init_name(),
            document: None,
        };
        return output;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Implementation {
    #[serde(skip)]
    pub name: String,
    impl_type: ImplementationType,
    #[serde(with = "crate::serde_serialization::use_name_for_arc_rwlock")]
    derived_streamlet: Arc<RwLock<Streamlet>>,
    nets: BTreeMap<String, Net>,
    implementation_instances: BTreeMap<String, ImplementationInstance>,
    document: Option<String>,
}

impl Implementation {
    pub fn new() -> Self {
        let output = Self {
            name: generate_init_name(),
            impl_type: ImplementationType::Unknown,
            derived_streamlet: Arc::new(RwLock::new(Streamlet::new())),
            nets: BTreeMap::new(),
            implementation_instances: BTreeMap::new(),
            document: None,
        };
        return output;
    }

    pub fn translate_from_tydi_project(tydi_project: Arc<RwLock<Project>>, target_var: Arc<RwLock<tydi_memory_representation::Variable>>) -> Result<(Arc<RwLock<Implementation>>, JsonRepresentation), String> {
        let var_value = target_var.read().unwrap().get_value();
        let parent_scope = target_var.read().unwrap().get_parent_scope().expect("bug: unknown parent scope");
        match &var_value {
            TypedValue::Implementation(target_implementation) => {
                return Self::translate_from_tydi_project_implementation(tydi_project.clone(), target_implementation.clone(), parent_scope.clone());
            },
            _ => unreachable!()
        }

    }

    pub fn translate_from_tydi_project_implementation(tydi_project: Arc<RwLock<Project>>, target_impl: Arc<RwLock<tydi_memory_representation::Implementation>>, scope: Arc<RwLock<Scope>>) -> Result<(Arc<RwLock<Implementation>>, JsonRepresentation), String> {
        let mut output_dependency = JsonRepresentation::new();
        let mut output_implementation = Implementation::new();

        let target_var_name = name_conversion::get_global_variable_name_with_parent_scope(target_impl.clone());
        output_implementation.name = target_var_name.clone();

        let implementation_scope = target_impl.read().unwrap().get_scope();
        //impl type
        {
            let impl_type = target_impl.read().unwrap().get_impl_type();
            match &impl_type {
                tydi_memory_representation::implementation::ImplementationType::Unknown => unreachable!(),
                tydi_memory_representation::implementation::ImplementationType::Normal => {
                    output_implementation.impl_type = ImplementationType::Normal;
                },
                tydi_memory_representation::implementation::ImplementationType::Template => {
                    unreachable!("template should never join the reference chain")
                },
                tydi_memory_representation::implementation::ImplementationType::TemplateInstance(template_impl, template_args) => {
                    let mut output_template_args = BTreeMap::new();
                    for (arg_index, arg) in template_args {
                        let (value, mut dependencies) = Value::translate_from_tydi_project(tydi_project.clone(), arg)?;
                        output_dependency.append(&mut dependencies);
                        output_template_args.insert(*arg_index, value);
                    }
                    output_implementation.impl_type = ImplementationType::TemplateInstance(template_impl.read().unwrap().get_name(), output_template_args);
                },
            }
        }

        //derived streamlet
        {
            let derived_streamlet = target_impl.read().unwrap().get_derived_streamlet().expect("bug: implementation with unknown streamlet");
            let (streamlet, mut dependencies) = Streamlet::translate_from_tydi_project_streamlet(tydi_project.clone(), derived_streamlet.clone(), implementation_scope.clone())?;
            output_dependency.append(&mut dependencies);
            output_implementation.derived_streamlet = streamlet.clone();
        }

        //document
        {
            output_implementation.document = target_impl.read().unwrap().get_document();
        }

        //instances and nets
        {
            let all_variables = implementation_scope.read().unwrap().get_variables();
            for (_, var) in all_variables {
                let var_value = var.read().unwrap().get_value();
                match var_value {
                    TypedValue::Instance(inst) => {
                        let inst_name = inst.read().unwrap().get_name();
                        if inst_name == "self" {
                            continue;   //ignore self
                        }
                        let inst_name = get_global_variable_name_with_parent_scope(inst.clone());
                        let inst_impl = inst.read().unwrap().get_derived_impl().expect("bug: instance without derived implementation");
                        let (instance_impl, mut dependencies) = Self::translate_from_tydi_project_implementation(tydi_project.clone(), inst_impl.clone(), implementation_scope.clone())?;
                        output_dependency.append(&mut dependencies);
                        
                        let mut output_instance = ImplementationInstance::new();
                        output_instance.name = inst_name.clone();
                        output_instance.derived_implementation = instance_impl.read().unwrap().name.clone();
                        output_instance.document = inst.read().unwrap().get_document();
                        output_implementation.implementation_instances.insert(output_instance.name.clone(), output_instance);
                    },
                    TypedValue::Net(net) => {
                        let mut output_net = Net::new();
                        let net_name = get_global_variable_name_with_scope(net.clone(), implementation_scope.clone());
                        output_net.name = net_name;
                        output_net.document = net.read().unwrap().get_document();

                        {
                            let src_port = net.read().unwrap().get_source_port().expect("bug: src port not available");
                            let src_port_parent_streamlet = src_port.read().unwrap().get_parent_streamlet().expect("bug: parent streamlet not available");
                            let src_port_name = get_global_variable_name_with_parent_scope(src_port.clone());
                            output_net.src_port_name = src_port_name;
                        }
                        {
                            let sink_port = net.read().unwrap().get_sink_port().expect("bug: sink port not available");
                            let sink_port_parent_streamlet = sink_port.read().unwrap().get_parent_streamlet().expect("bug: parent streamlet not available");
                            let sink_port_name = get_global_variable_name_with_parent_scope(sink_port.clone());
                            output_net.sink_port_name = sink_port_name;
                        }

                        //set port owner
                        let convert_port_owner_to_string = |owner: tydi_memory_representation::PortOwner| -> String {
                            return match owner {
                                PortOwner::Unknown => unreachable!(),
                                PortOwner::ImplSelf => String::from("self"),
                                PortOwner::ImplInstance(impl_inst) => get_global_variable_name_with_parent_scope(impl_inst.clone()),
                            };
                        };

                        output_net.src_port_owner_name = convert_port_owner_to_string(net.read().unwrap().get_source_port_owner());
                        output_net.sink_port_owner_name = convert_port_owner_to_string(net.read().unwrap().get_sink_port_owner());

                        output_implementation.nets.insert(output_net.name.clone(), output_net);
                    },
                    _ => (),    //ignore
                }
            }
        }

        let output_implementation = Arc::new(RwLock::new(output_implementation));
        output_dependency.implementations.insert(target_var_name.clone(), output_implementation.clone());

        return Ok((output_implementation, output_dependency));
    }

}