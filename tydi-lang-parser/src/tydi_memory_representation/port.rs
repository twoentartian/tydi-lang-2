use std::sync::{Arc, RwLock};

use serde::{Serialize, Deserialize};

use crate::tydi_memory_representation::{Variable, Attribute, CodeLocation, TraitCodeLocationAccess};

use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_name, generate_access_pub, generate_get_pub, generate_set_pub};

use super::TypedValue;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PortDirection {
    In,
    Out,
    Unknown,
}

#[derive(Clone, Debug, Serialize)]
pub struct Port {
    name: String,

    direction: PortDirection,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    time_domain: Arc<RwLock<Variable>>,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    logical_type: Arc<RwLock<Variable>>,

    attributes: Vec<Attribute>,

    document: Option<String>,

    location_define: CodeLocation,
}

impl GetName for Port {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl HasDocument for Port {
    generate_access!(document, Option<String>, get_document, set_document);
}

impl TraitCodeLocationAccess for Port {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl Port {
    pub fn new(name: String, direction: PortDirection, logical_type: Arc<RwLock<Variable>>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            direction: direction,
            time_domain: Self::get_default_time_domain(),
            logical_type: logical_type,
            attributes: vec![],
            document: None,
            location_define: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn new_place_holder() -> Arc<RwLock<Self>> {
        let output = Self {
            name: generate_name::generate_init_value(),
            direction: PortDirection::Unknown,
            time_domain: Self::get_default_time_domain(),
            logical_type: Variable::new_place_holder(),
            attributes: vec![],
            document: None,
            location_define: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn get_default_time_domain() -> Arc<RwLock<Variable>> {
        return Variable::new_predefined(format!("!default_time_domain"), TypedValue::StringValue(format!("default_time_domain")));
    }

    generate_access_pub!(time_domain, Arc<RwLock<Variable>>, get_time_domain, set_time_domain);
    generate_access_pub!(logical_type, Arc<RwLock<Variable>>, get_logical_type, set_logical_type);
    generate_access_pub!(direction, PortDirection, get_direction, set_direction);
    generate_access_pub!(attributes, Vec<Attribute>, get_attributes, set_attributes);
}
