use std::sync::{Arc, RwLock};

use serde::{Serialize, Deserialize};

use crate::deep_clone::DeepClone;
use crate::tydi_memory_representation::{Variable, Attribute, CodeLocation, TraitCodeLocationAccess, TypedValue, Streamlet};

use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_name, generate_access_pub, generate_get_pub, generate_set_pub};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PortDirection {
    In,
    Out,
    Unknown,
}

impl DeepClone for PortDirection {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl PortDirection {
    pub fn to_string(&self) -> String {
        return match self {
            PortDirection::In => String::from("In"),
            PortDirection::Out => String::from("Out"),
            PortDirection::Unknown => String::from("Unknown"),
        };
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Port {
    name: String,

    direction: PortDirection,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    time_domain: Arc<RwLock<Variable>>,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    logical_type: Arc<RwLock<Variable>>,

    #[serde(with = "crate::serde_serialization::use_name_for_optional_arc_rwlock")]
    parent_streamlet: Option<Arc<RwLock<Streamlet>>>,

    attributes: Vec<Attribute>,

    document: Option<String>,

    location_define: CodeLocation,
}

impl GetName for Port {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl DeepClone for Port {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.deep_clone(),
            direction: self.direction.deep_clone(),
            time_domain: self.time_domain.deep_clone(),
            logical_type: self.logical_type.deep_clone(),
            parent_streamlet: None,
            attributes: self.attributes.deep_clone(),
            document: self.document.deep_clone(),
            location_define: self.location_define.deep_clone(),
        };
        return output;
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
            parent_streamlet: None,
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
            parent_streamlet: None,
            attributes: vec![],
            document: None,
            location_define: CodeLocation::new_unknown(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn get_default_time_domain() -> Arc<RwLock<Variable>> {
        return Variable::new_predefined(format!("!default_time_domain"), TypedValue::StringValue(format!("default_time_domain")));
    }

    generate_set_pub!(name, String, set_name);
    generate_access_pub!(time_domain, Arc<RwLock<Variable>>, get_time_domain, set_time_domain);
    generate_access_pub!(logical_type, Arc<RwLock<Variable>>, get_logical_type, set_logical_type);
    generate_access_pub!(parent_streamlet, Option<Arc<RwLock<Streamlet>>>, get_parent_streamlet, set_parent_streamlet);
    generate_access_pub!(direction, PortDirection, get_direction, set_direction);
    generate_access_pub!(attributes, Vec<Attribute>, get_attributes, set_attributes);
}
