use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};

use crate::deep_clone::{DeepClone, DeepClone_ArcLock};
use crate::tydi_memory_representation::{TemplateArg, CodeLocation, Scope, ScopeType, GetScope, Attribute, TraitCodeLocationAccess, TypedValue, GlobalIdentifier, Port};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub, generate_name};

#[derive(Clone, Debug, Serialize)]
pub struct Streamlet {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    scope: Arc<RwLock<Scope>>,

    location_define: CodeLocation,

    #[serde(with = "crate::serde_serialization::use_name_for_optional_arc_rwlock")]
    parent_scope: Option<Arc<RwLock<Scope>>>,
    id_in_scope: Option<String>,

    document: Option<String>,

    template_args: Option<BTreeMap<usize, TemplateArg>>,

    attributes: Vec<Attribute>,
}

impl GetName for Streamlet {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl DeepClone for Streamlet {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            scope: self.scope.read().unwrap().deep_clone_arclock(),
            location_define: self.location_define.deep_clone(),
            parent_scope: self.parent_scope.clone(),
            id_in_scope: self.id_in_scope.clone(),
            document: self.document.deep_clone(),
            template_args: self.template_args.deep_clone(),
            attributes: self.attributes.deep_clone(),
        };
        return output;
    }
}

impl HasDocument for Streamlet {
    generate_access!(document, Option<String>, get_document, set_document);
}

impl TraitCodeLocationAccess for Streamlet {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl GetScope for Streamlet {
    generate_get!(scope, Arc<RwLock<Scope>>, get_scope);
}

impl GlobalIdentifier for Streamlet {
    generate_access!(parent_scope, Option<Arc<RwLock<Scope>>>, get_parent_scope, set_parent_scope);
    generate_access!(id_in_scope, Option<String>, get_id_in_scope, set_id_in_scope);
}

impl Streamlet {
    pub fn new(name: String, parent_scope: Arc<RwLock<Scope>>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name.clone(),
            scope: Scope::new(format!("streamlet_{}", name.clone()), ScopeType::StreamletScope, parent_scope.clone()),
            location_define: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            document: None,
            template_args: None,
            attributes: vec![],
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_place_holder() -> Arc<RwLock<Self>> {
        let output = Self {
            name: generate_name::generate_init_value(),
            scope: Scope::new_place_holder(),
            location_define: CodeLocation::new_unknown(),
            parent_scope: None,
            id_in_scope: None,
            document: None,
            template_args: None,
            attributes: vec![],
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name.clone();
        self.scope.write().unwrap().set_name(format!("streamlet_{}", name.clone()));
    }

    pub fn get_brief_info(&self) -> String {
        let mut ports_strs = vec![];
        let streamlet_vars = self.scope.read().unwrap().get_variables();
        for (_, var) in streamlet_vars {
            if let TypedValue::Port(port) = var.read().unwrap().get_value() {
                let logic_type = port.read().unwrap().get_logical_type();
                ports_strs.push(format!("{}:{} {}", var.read().unwrap().get_name(), port.read().unwrap().get_direction().to_string(), logic_type.read().unwrap().get_name()));
            }
        }

        return format!("Streamlet({}){{{}}}", &self.name, ports_strs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", "));
    }

    generate_access_pub!(template_args, Option<BTreeMap<usize, TemplateArg>>, get_template_args, set_template_args);
    generate_access_pub!(attributes, Vec<Attribute>, get_attributes, set_attributes);
}

//interfaces for quick access
impl Streamlet {
    pub fn get_all_ports(&self) -> Vec<Arc<RwLock<Port>>> {
        let scope = self.scope.clone();
        let vars = scope.read().unwrap().get_variables();
        let mut all_ports = vec![];
        for (var_name, var) in vars {
            let var_value = var.read().unwrap().get_value();
            let mut ports = match &var_value {
                crate::tydi_memory_representation::TypedValue::Port(p) => Some(vec![p.clone()]),
                crate::tydi_memory_representation::TypedValue::Array(a) => {
                    let mut output = vec![];
                    for v in a {
                        match v {
                            crate::tydi_memory_representation::TypedValue::Port(p) => {
                                output.push(p.clone());
                            },
                            _ => (),
                        }
                    }
                    Some(output)
                },
                _ => None,
            };
            match &mut ports {
                Some(ps) => all_ports.append(ps),
                None => (),
            }
        }

        return all_ports;
    }
}
