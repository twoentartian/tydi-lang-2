#[test]
fn save_load_project_description(){
    let mut project_description = crate::project_make::ProjectDescription::generate_default();
    let toml_text = project_description.to_toml();
    println!("{toml_text}");
    let toml_text = toml_text.replace("tydi_src_", "new_src_");
    println!("{toml_text}");
    project_description.apply_toml(toml_text).expect("cannot parse toml");
    println!("{project_description:?}");



}