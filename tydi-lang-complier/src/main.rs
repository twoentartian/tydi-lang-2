use std::path::{PathBuf};

use project::TydiProject;

use crate::project_description::ProjectDescription;

extern crate toml;

mod project;
mod project_description;

mod test;

pub fn main() {
    let project_description_file_path_string = match std::env::args().nth(1) {
        Some(v) => v,
        None => format!("./sample_tydi_project.toml"),      //default description path
    };
    let description_file_path = PathBuf::from(&project_description_file_path_string);
    if !description_file_path.exists() {
        let default_description = ProjectDescription::generate_default();
        let default_description_file_content = default_description.to_toml();
        std::fs::write(description_file_path.clone(), default_description_file_content).expect("cannot write to default project description file");
        panic!("no project description file provided, the tydi-lang complier creates a default description file in {}", &project_description_file_path_string);
    }

    let project_description_file_path = PathBuf::from(project_description_file_path_string.clone());
    let project_description_file_content = std::fs::read_to_string(project_description_file_path).expect(&format!("cannot read the default project description file: {}", project_description_file_path_string.clone()));
    let project_description = ProjectDescription::from_toml(project_description_file_content);
    if project_description.is_err() {
        let err = project_description.err().unwrap();
        panic!("{}", err);
    }

    let project_description = &project_description.unwrap();
    let output_folder = &PathBuf::from(&project_description.output_path);
    if !output_folder.exists() {
        std::fs::create_dir(output_folder).expect("cannot create output folder");
    }

    let tydi_project = TydiProject::load_project_description(project_description);
    if tydi_project.is_err() {
        let err = tydi_project.err().unwrap();
        panic!("{}", err);
    }
    let tydi_project = tydi_project.unwrap();

    // parse project
    let result = tydi_project.parse();
    if result.is_err() {
        let err = result.err().unwrap();
        panic!("fail to parse project, error:{}", err);
    }
    let result = result.unwrap();
    println!("{}", result);

    // save parsing result to file
    std::fs::write(output_folder.join("parser_result.json"), tydi_project.get_pretty_json()).expect("cannot write parser_result.json");

    return;
}