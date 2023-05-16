use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use serde::{Serialize};

use crate::error::TydiLangError;
use crate::tydi_memory_representation::Package;
use crate::tydi_lang_src_to_memory_representation::tydi_lang_src_to_memory_representation;

#[derive(Clone, Debug, Serialize)]
pub struct Project {
    name: String,

    #[serde(skip)]
    src_files: HashMap<String, String>,

    #[serde(with = "crate::serde_serialization::arc_rwlock_in_hash_map_value")]
    packages: HashMap<String, Arc<RwLock<Package>>>,
}

impl Project {
    pub fn new(name: String) -> Arc<RwLock<Self>> {
        let output = Project { 
            name: name, 
            src_files: HashMap::new(),
            packages: HashMap::new(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn parse_package(&mut self, file_path: String, file_content: String) -> Result<(), TydiLangError> {
        let file_package = crate::tydi_lang_src_to_memory_representation::tydi_lang_src_to_memory_representation(file_content.clone())?;
        {
            let mut file_package_write = file_package.write().unwrap();
            file_package_write.set_file_path(file_path);
        }
        let package_name = file_package.read().unwrap().get_name();
        self.packages.insert(package_name.clone(), file_package);
        self.src_files.insert(package_name.clone(), file_content.clone());
        return Ok(());
    }

    pub fn get_pretty_json(&self) -> String {
        let json_output = serde_json::to_string_pretty(self).ok().unwrap();
        return json_output;
    }
}