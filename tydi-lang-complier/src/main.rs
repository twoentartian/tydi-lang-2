use std::{path::PathBuf, sync::{Arc, RwLock}};

use clap::{Parser, command, ArgAction, Arg};

use project::TydiProject;
use tydi_lang_parser::post_compile::sugaring_auto_insertion_duplicator_voider;

use crate::project_description::ProjectDescription;

extern crate toml;

mod project;
mod project_description;

mod test;


#[derive(Parser, Debug)]
#[command(author, version, about="compile a Tydi project", long_about = None)]
struct Args {
    /// Name of the Tydi project
    #[arg(short='n', long)]
    name: Option<String>,

    /// Specify an output path
    #[arg(short='o', long)]
    output: Option<String>,

    /// Config file path, CLI options override config file items.
    #[arg(short='c', long, value_name = "FILE")]
    config_file: Option<PathBuf>,

    /// Name of the top-level component
    #[arg(short='i', long)]
    top_level_implementation: Option<String>,

    /// Name of the top-level package
    #[arg(short='p', long)]
    top_level_implementation_package: Option<String>,

    /// Tydi source files, can have multiple values
    #[arg(short='f', long)]
    source: Vec<String>,

    /// Sugaring the project - auto insertions of duplicators and voiders
    #[arg(short, long)]
    sugaring: bool,
}

pub fn main() {
    let args = Args::parse();

    let mut project_description = ProjectDescription::generate_default();

    let default_tydi_project_description_file_path = "./sample_tydi_project.toml";
    match &args.config_file {
        None => {
            let default_description = ProjectDescription::generate_default();
            let default_description_file_content = default_description.to_toml();
            std::fs::write(default_tydi_project_description_file_path.clone(), default_description_file_content).expect("cannot write to default project description file");
            println!("no project description file provided, the tydi-lang complier creates a default description file in {}", default_tydi_project_description_file_path);
        }
        Some(project_description_file_path) => {
            let project_description_file_content = std::fs::read_to_string(project_description_file_path.clone()).expect(&format!("cannot read the default project description file: {}", project_description_file_path.clone().into_os_string().into_string().unwrap()));
            let project_description_result = ProjectDescription::from_toml(project_description_file_content);
            if project_description_result.is_err() {
                let err = project_description_result.err().unwrap();
                panic!("{}", err);
            }
            project_description = project_description_result.unwrap();
        }
    }

    //override project description with command line args
    match &args.name {
        Some(project_name) => {
            project_description.properties.name = project_name.clone();
        },
        None => (),
    }
    match &args.output {
        Some(output_path) => {
            project_description.output_path = output_path.clone();
        },
        None => (),
    }
    match &args.top_level_implementation {
        Some(implementation) => {
            project_description.properties.top_level_implementation = implementation.clone();
        },
        None => (),
    }
    match &args.top_level_implementation_package {
        Some(package) => {
            project_description.properties.top_level_implementation_package = package.clone();
        },
        None => (),
    }
    if !args.source.is_empty() {
        for src in args.source {
            project_description.files.tydi_src.push(src.clone());
        }
    }
    if args.sugaring {
        project_description.properties.sugaring = true;
    }

    // begin to compile
    let output_folder = &PathBuf::from(&project_description.output_path);
    if !output_folder.exists() {
        std::fs::create_dir(output_folder).expect("cannot create output folder");
    }

    let tydi_project = TydiProject::load_project_description(&project_description);
    if tydi_project.is_err() {
        let err = tydi_project.err().unwrap();
        panic!("{}", err);
    }
    let tydi_project = tydi_project.unwrap();

    // parse project
    println!("parsing project");
    let result = tydi_project.parse();
    if result.is_err() {
        let err = result.err().unwrap();
        panic!("fail to parse project, error:{}", err);
    }
    std::fs::write(output_folder.join("parser_result.json"), tydi_project.get_pretty_json()).expect("cannot write parser_result.json");

    // evaluation
    println!("evaluate project from {} in {}", &project_description.properties.top_level_implementation, &project_description.properties.top_level_implementation_package);
    let result = tydi_project.evaluation(project_description.properties.top_level_implementation.clone(), project_description.properties.top_level_implementation_package.clone());
    if result.is_err() {
        let err = result.err().unwrap();
        panic!("fail to evaluate project, error:{}", err);
    }
    std::fs::write(output_folder.join("code_structure.json"), tydi_project.get_pretty_json()).expect("cannot write code_structure.json");

    // sugaring?
    if project_description.properties.sugaring {
        let result = tydi_project.sugaring(project_description.properties.top_level_implementation.clone(), project_description.properties.top_level_implementation_package.clone());
        if result.is_err() {
            let err = result.err().unwrap();
            panic!("fail to sugaring project, error:{}", err);
        }
    }

    // generate json IR
    println!("generate json IR");
    let json_output = tydi_project.generate_json_IR(project_description.properties.top_level_implementation.clone(), project_description.properties.top_level_implementation_package.clone()).expect("fail to generate json");
    std::fs::write(output_folder.join("json_IR.json"), json_output).expect("cannot write json_IR.json");

    return;
}