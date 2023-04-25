use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::tydi_memory_representation::{Scope, ScopeType, CodeLocation, TraitCodeLocationAccess};
use crate::trait_common::{GetName, HasDocument};
use crate::{generate_access, generate_get, generate_set};

#[derive(Clone, Debug, Serialize)]
pub struct LogicUnion {
    name: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    scope: Arc<RwLock<Scope>>,

    #[serde(skip_serializing)]
    location_define: CodeLocation,

    document: Option<String>,
}

impl GetName for LogicUnion {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl HasDocument for LogicUnion {
    generate_access!(document, Option<String>, get_document, set_document);
}

impl TraitCodeLocationAccess for LogicUnion {
    generate_access!(location_define, CodeLocation, get_code_location, set_code_location);
}

impl LogicUnion {
    fn new(name: String, parent_scope: Arc<RwLock<Scope>>) -> Arc<RwLock<Self>> {
        let output = Self {
            name: name.clone(),
            scope: Scope::new(format!("logic_union_{}", name.clone()), ScopeType::GroupScope, parent_scope.clone()),
            location_define: CodeLocation::new_unknown(),
            document: None,
        };
        return Arc::new(RwLock::new(output));
    }
}
