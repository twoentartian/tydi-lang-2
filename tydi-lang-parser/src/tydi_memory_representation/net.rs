use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::tydi_memory_representation::{CodeLocation, Attribute, TraitCodeLocationAccess, Variable};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub};

#[derive(Clone, Debug, Serialize)]
pub struct Net {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    source: Arc<RwLock<Variable>>,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    sink: Arc<RwLock<Variable>>,

    #[serde(with = "crate::serde_serialization::use_inner_for_optional_arc_rwlock")]
    net_name: Option<Arc<RwLock<Variable>>>,

    location_define: CodeLocation,

    document: Option<String>,

    attributes: Vec<Attribute>,
}

impl GetName for Net {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl HasDocument for Net {
    generate_access!(document, Option<String>, get_document, set_document);
}

impl TraitCodeLocationAccess for Net {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl Net {
    pub fn new(name: String) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name,
            source: Variable::new_place_holder(),
            sink: Variable::new_place_holder(),
            net_name: None,
            location_define: CodeLocation::new_unknown(),
            document: None,
            attributes: vec![],
        };
        return Arc::new(RwLock::new(output));
    }

    generate_set_pub!(name, String, set_name);
    generate_access_pub!(attributes, Vec<Attribute>, get_attributes, set_attributes);
    generate_access_pub!(source, Arc<RwLock<Variable>>, get_source, set_source);
    generate_access_pub!(sink, Arc<RwLock<Variable>>, get_sink, set_sink);
    generate_access_pub!(net_name, Option<Arc<RwLock<Variable>>>, get_net_name, set_net_name);
}