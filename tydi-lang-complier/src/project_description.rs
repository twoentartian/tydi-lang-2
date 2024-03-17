use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectProperties {
    pub name: String,
    pub top_level_implementation: String,
    pub top_level_implementation_package: String,
    pub sugaring: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFiles {
    pub tydi_src: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDescription {
    pub properties: ProjectProperties,
    pub files: ProjectFiles,
    pub output_path: String,
}

impl ProjectDescription {
    pub fn generate_default() -> Self {
        return Self {
            properties: ProjectProperties {
                sugaring: false,
                name: format!("sample_tydi_project"),
                top_level_implementation: format!("sample_target"),
                top_level_implementation_package: format!("sample_pack"),
            },
            files: ProjectFiles{
                tydi_src: vec![format!("./tydi_src_0.td"), format!("./tydi_src_1.td")],
            },
            output_path: format!("./output"),
        };
    }

    pub fn to_toml(&self) -> String {
        let toml_text = toml::to_string(self).unwrap();
        return toml_text;
    }

    pub fn apply_toml(&mut self, toml_text: String) -> Result<(), String> {
        let result = toml::from_str::<Self>(&toml_text);
        if result.is_err() {
            return Err(result.err().unwrap().to_string());
        }
        *self = result.ok().unwrap();
        return Ok(());
    }

    pub fn from_toml(toml_text: String) -> Result<Self, String> {
        let result = toml::from_str::<Self>(&toml_text);
        if result.is_err() {
            return Err(result.err().unwrap().to_string());
        }
        return Ok(result.ok().unwrap());
    }

}