use std::path::{PathBuf};

use crate::project_make::*;

pub struct Project {
    name: String,
    toplevel_implementation: String,
    src_files: Vec<PathBuf>,

    

}

impl Project {
    pub fn load_project_description(description: &ProjectDescription) -> Result<Self, String> {
        let mut src_paths: Vec<PathBuf> = Vec::new();
        for src in &description.files.tydi_src {
            let mut src_path = PathBuf::new();
            src_path.push(src);
            if src_path.exists() {
                src_paths.push(src_path);
            }
            else {
                return Err(format!("file {} does not exist", src));
            }
        }

        return Ok(Self {
            name: description.properties.name.clone(),
            toplevel_implementation: description.properties.top_level_implementation.clone(),
            src_files: src_paths,
        });
    }

}

