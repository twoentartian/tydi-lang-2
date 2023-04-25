use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::tydi_memory_representation::{Scope, ScopeType, CodeLocation, TraitCodeLocationAccess};

use crate::trait_common::GetName;

#[derive(Clone, Debug, Serialize)]
pub struct LogicStream {
    name: String,

    #[serde(skip_serializing)]
    location_define: CodeLocation,
}

impl GetName for LogicStream {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}


