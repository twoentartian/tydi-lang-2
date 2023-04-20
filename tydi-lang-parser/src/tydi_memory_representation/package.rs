use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::name_trait::GetName;
use crate::tydi_memory_representation::{Scope};

#[derive(Clone, Debug, Serialize)]
pub struct Package {
    name: String,

    #[serde(with = "crate::serde_arc_rwlock_name")]
    package_scope: Arc<RwLock<Scope>>
}

impl GetName for Package {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl Package {
    pub fn new(name: String) -> Arc<RwLock<Self>> {
        let package_scope = Scope::new_top_scope(format!("package_{name}"));
        return Arc::new(RwLock::new(Self {
            name: name.clone(),
            package_scope: package_scope,
        }));
    }
}