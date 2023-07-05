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

use tydi_lang_parser::tydi_memory_representation::{Project, scope::GetScope};

fn generate_json_representation_from_tydi_project(project: Arc<RwLock<Project>>, target_name: String, package_name: String) -> Result<String, String> {
    let mut project_json = json_representation_all::JsonRepresentation::new();

    let target_var = project.read().unwrap().get_variable(package_name, target_name)?;
    let (top_level_type, result_json_representation) = json_representation_all::translate_from_tydi_project(project, target_var.clone())?;

    let json_output = serde_json::to_string_pretty(&result_json_representation).expect("fail to convert the JsonRepresentation to json string");
    return Ok(json_output);
}