use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::deep_clone::{DeepClone, DeepClone_ArcLock};
use crate::tydi_memory_representation::{CodeLocation, Attribute, TraitCodeLocationAccess, Variable, Instance, Streamlet, Implementation, Port};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub};

#[derive(Clone, Debug, Serialize)]
pub struct Net {
    name: String,

    #[serde(skip)]
    source: Arc<RwLock<Variable>>,
    #[serde(with = "crate::serde_serialization::use_inner_for_optional_arc_rwlock")]
    source_port: Option<Arc<RwLock<Port>>>,


    #[serde(skip)]
    sink: Arc<RwLock<Variable>>,
    #[serde(with = "crate::serde_serialization::use_inner_for_optional_arc_rwlock")]
    sink_port: Option<Arc<RwLock<Port>>>,

    #[serde(with = "crate::serde_serialization::use_inner_for_optional_arc_rwlock")]
    net_name: Option<Arc<RwLock<Variable>>>,

    #[serde(with = "crate::serde_serialization::use_name_for_optional_arc_rwlock")]
    parent_impl: Option<Arc<RwLock<Implementation>>>,

    location_define: CodeLocation,

    document: Option<String>,

    attributes: Vec<Attribute>,
}

impl GetName for Net {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl DeepClone for Net {
    fn deep_clone(&self) -> Self {
        let output = Self {
            name: self.name.clone(),
            source: self.source.read().unwrap().deep_clone_arclock(),
            source_port: self.source_port.deep_clone(),
            sink: self.sink.read().unwrap().deep_clone_arclock(),
            sink_port: self.sink_port.deep_clone(),
            net_name: self.net_name.deep_clone(),
            parent_impl: self.parent_impl.deep_clone(),
            location_define: self.location_define.deep_clone(),
            document: self.document.deep_clone(),
            attributes: self.attributes.deep_clone(),
        };
        return output;
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
            source_port: None,
            
            sink: Variable::new_place_holder(),
            sink_port: None,

            net_name: None,
            parent_impl: None,
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
    generate_access_pub!(source_port, Option<Arc<RwLock<Port>>>, get_source_port, set_source_port);
    generate_access_pub!(sink_port, Option<Arc<RwLock<Port>>>, get_sink_port, set_sink_port);
    generate_access_pub!(net_name, Option<Arc<RwLock<Variable>>>, get_net_name, set_net_name);
    generate_access_pub!(parent_impl, Option<Arc<RwLock<Implementation>>>, get_parent_impl, set_parent_impl);
}