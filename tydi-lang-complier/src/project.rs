use std::path::{PathBuf};

use std::sync::{Arc, RwLock};

use crate::project_description::*;
use tydi_lang_parser::tydi_memory_representation::*;

pub struct TydiProject {
    name: String,
    toplevel_implementation: String,
    src_files: Vec<PathBuf>,

    project: Arc<RwLock<Project>>,
}

impl TydiProject {
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
            project: Project::new(description.properties.name.clone()),
        });
    }

    pub fn parse(&self) -> Result<String, String> {
        let mut project_write = self.project.write().unwrap();
        let mut output_message = String::new();
        for single_src in &self.src_files {
            let file_path = single_src.to_str().unwrap().to_string();
            let file_content = std::fs::read_to_string(single_src).expect(&format!("cannot read file {}", single_src.to_string_lossy()));
            let add_package_result = project_write.add_package(file_path.clone(), file_content.clone());
            if add_package_result.is_err() {
                let src_pointer = Arc::new(RwLock::new(file_content.clone()));
                let err = add_package_result.err().unwrap();
                return Err(err.print(Some(src_pointer)));
            }
            output_message.push_str(&format!("parse finished: {}\n", single_src.as_os_str().to_str().unwrap().to_string()));
        }
        return Ok(output_message);
    }

    pub fn evaluation(&self) -> Result<String, String> {
        todo!()
    }

    pub fn get_pretty_json(&self) -> String {
        return self.project.read().unwrap().get_pretty_json();
    }


}

