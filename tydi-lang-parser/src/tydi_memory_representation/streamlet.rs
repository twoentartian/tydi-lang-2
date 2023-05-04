use std::sync::{Arc, RwLock};
use std::collections::BTreeMap;

use serde::{Serialize};

use crate::tydi_memory_representation::{TemplateArg, CodeLocation, Scope, ScopeType, GetScope, Attribute, TraitCodeLocationAccess};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set, generate_access_pub, generate_get_pub, generate_set_pub, generate_name};

#[derive(Clone, Debug, Serialize)]
pub struct Streamlet {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    scope: Arc<RwLock<Scope>>,

    location_define: CodeLocation,

    document: Option<String>,

    template_args: Option<BTreeMap<usize, TemplateArg>>,

    attributes: Vec<Attribute>,
}

impl GetName for Streamlet {
    fn get_name(&self) -> String {
        return self.name.clone();
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

impl Streamlet {
    pub fn new(name: String, parent_scope: Arc<RwLock<Scope>>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name.clone(),
            scope: Scope::new(format!("streamlet_{}", name.clone()), ScopeType::StreamletScope, parent_scope.clone()),
            location_define: CodeLocation::new_unknown(),
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

    generate_access_pub!(template_args, Option<BTreeMap<usize, TemplateArg>>, get_template_args, set_template_args);
    generate_access_pub!(attributes, Vec<Attribute>, get_attributes, set_attributes);
}