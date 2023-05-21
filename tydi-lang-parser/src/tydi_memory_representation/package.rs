use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::generate_name::generate_init_value;
use crate::{generate_set_pub, generate_get_pub, generate_access_pub};
use crate::trait_common::{GetName};
use crate::tydi_memory_representation::{Scope, CodeLocation, TraitCodeLocationAccess, GetScope};

#[derive(Clone, Debug, Serialize)]
pub struct Package {
    name: String,

    file_path: String,

    file_content: String,

    #[serde(with = "crate::serde_serialization::use_inner_for_arc_rwlock")]
    package_scope: Arc<RwLock<Scope>>,

    pub location_define: CodeLocation,
}

impl GetName for Package {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl GetScope for Package {
    fn get_scope(&self) -> Arc<RwLock<Scope>> {
        return self.package_scope.clone();
    }
}

impl TraitCodeLocationAccess for Package {
    fn set_code_location(& mut self, loc: CodeLocation) {
        self.location_define = loc;
    }

    fn get_code_location(&self) -> CodeLocation {
        return self.location_define.clone();
    }
}

impl Package {
    pub fn new(name: String) -> Arc<RwLock<Self>> {
        let package_scope = Scope::new_top_scope(format!("package_{name}"));
        return Arc::new(RwLock::new(Self {
            name: name.clone(),
            file_path: generate_init_value(),
            file_content: generate_init_value(),
            package_scope: package_scope,
            location_define: CodeLocation::new_unknown(),
        }));
    }

    pub fn set_name(& mut self, name: String) {
        self.name = name.clone();
        self.package_scope.write().unwrap().set_name(format!("package_{}", name.clone()));
    }

    generate_get_pub!(name, String, get_name);
    generate_access_pub!(file_path, String, get_file_path, set_file_path);
    generate_access_pub!(file_content, String, get_file_content, set_file_content);
}