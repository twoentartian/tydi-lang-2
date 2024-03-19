mod json_representation_logic_type;
mod json_representation_implementation;
mod json_representation_streamlet;
mod json_representation_value;
mod json_representation_all;

mod name_conversion;
mod global_values;
mod serde_serialization;
mod util;

mod test_project;


use std::sync::{Arc, RwLock};

use tydi_lang_parser::tydi_memory_representation::Project;

pub fn generate_json_representation_from_tydi_project(project: Arc<RwLock<Project>>, target_name: String, package_name: String) -> Result<String, String> {
    let result_json_representation = generate_json_target_from_tydi_project(project, target_name, package_name).expect("fail to generate JsonRepresentation");
    let json_output = serde_json::to_string_pretty(&result_json_representation).expect("fail to convert the JsonRepresentation to json string");
    return Ok(json_output);
}

pub fn generate_json_target_from_tydi_project(project: Arc<RwLock<Project>>, target_name: String, package_name: String) -> Result<json_representation_all::JsonRepresentation, String> {
    let target_var = project.read().unwrap().get_variable(package_name.clone(), target_name.clone())?;
    let (_, mut result_json_representation) = json_representation_all::translate_from_tydi_project(project.clone(), target_var.clone())?;
    result_json_representation.compile_options.top_level_implementation = target_name.clone();
    result_json_representation.compile_options.package_of_top_level_implementation = package_name.clone();
    result_json_representation.compile_options.sugaring_list = project.read().unwrap().get_sugaring_entry_point();

    let all_packages = project.read().unwrap().get_packages();
    for (_, package) in all_packages {
        result_json_representation.compile_options.packages_and_source_files.insert(package.read().unwrap().get_name(), package.read().unwrap().get_file_path());
    }

    return Ok(result_json_representation);
}