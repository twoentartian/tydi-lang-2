use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectProperties {
    pub name: String,
    pub top_level_implementation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFiles {
    pub tydi_src: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDescription {
    pub properties: ProjectProperties,
    pub files: ProjectFiles,
}

impl ProjectDescription {
    pub fn generate_default() -> Self {
        return Self {
            properties: ProjectProperties {
                name: format!("tydi_project_example"),
                top_level_implementation: format!("implementation_0")
            },
            files: ProjectFiles{
                tydi_src: vec![format!("./tydi_src_0.td"), format!("./tydi_src_1.td")],
            },
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