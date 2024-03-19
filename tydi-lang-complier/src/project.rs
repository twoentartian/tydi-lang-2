use std::path::PathBuf;

use std::sync::{Arc, RwLock};

use crate::project_description::*;
use tydi_lang_parser::tydi_memory_representation::*;
use tydi_lang_json_generator::generate_json_representation_from_tydi_project;
use tydi_lang_parser::post_compile::sugaring_auto_insertion_duplicator_voider;

pub struct TydiProject {
    name: String,
    toplevel_implementation: String,
    top_level_implementaton_package: String,

    src_files: Vec<PathBuf>,
    output_path: String,

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
        let output = Self {
            name: description.properties.name.clone(),
            toplevel_implementation: description.properties.top_level_implementation.clone(),
            top_level_implementaton_package: description.properties.top_level_implementation_package.clone(),
            src_files: src_paths,
            output_path: description.output_path.clone(),
            project: Project::new(description.properties.name.clone()),
        };

        return Ok(output);
    }

    pub fn parse(&self) -> Result<String, String> {
        let mut project_write = self.project.write().unwrap();
        let mut output_message = String::new();
        for single_src in &self.src_files {
            let file_path = single_src.to_str().unwrap().to_string();
            let file_content = std::fs::read_to_string(single_src).expect(&format!("cannot read file {}", single_src.to_string_lossy()));
            let add_package_result = project_write.add_package(file_path.clone(), file_content.clone());
            if add_package_result.is_err() {
                let err = add_package_result.err().unwrap();
                return Err(err.print());
            }
            output_message.push_str(&format!("parse finished: {}\n", single_src.as_os_str().to_str().unwrap().to_string()));
        }
        return Ok(output_message);
    }

    pub fn evaluation(&self, package_name: String, target_name: String) -> Result<String, String> {
        let result = self.project.read().unwrap().evaluate_target(package_name, target_name);
        match result {
            Ok(evaluator) => {
                return Ok(evaluator.read().unwrap().print_evaluation_record());
            },
            Err(err) => {
                return Err(err.print());
            }
        }
    }

    pub fn sugaring(&self, package_name: String, target_name: String) -> Result<String, String> {
        let result = sugaring_auto_insertion_duplicator_voider::sugaring_add_duplicator_voider(self.project.clone(), target_name, package_name);
        match result {
            Ok(evaluator) => {
                return Ok(evaluator.read().unwrap().print_evaluation_record());
            },
            Err(err) => {
                return Err(err.print());
            }
        }
    }

    pub fn generate_json_IR(&self, target_name: String, package_name: String) -> Result<String, String> {
        let tydi_project = self.project.clone();
        let json_output = generate_json_representation_from_tydi_project(tydi_project, target_name, package_name);
        return json_output;
    }

    pub fn get_pretty_json(&self) -> String {
        return self.project.read().unwrap().get_pretty_json();
    }


}

