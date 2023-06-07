use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use std::backtrace::Backtrace;

use crate::tydi_memory_representation::CodeLocation;

#[derive(Clone)]
pub struct TydiLangError {
    pub message: String,
    pub location: Vec<CodeLocation>,
    pub stack_trace: Arc<Backtrace>,
}

impl Debug for TydiLangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("message: {}\n", &self.message))?;
        f.write_str(&format!("location: {:?}\n", &self.location))?;
        f.write_str(&format!("stack trace: {:#?}\n", &self.stack_trace.to_owned()))?;
        return Ok(());
    }
}

impl TydiLangError {
    pub fn new(message: String, location: CodeLocation) -> Self {
        return Self {
            message: message,
            location: vec![location],
            stack_trace: Arc::new(Backtrace::capture()),
        };
    }

    pub fn new_multiple_locations(message: String, locations: Vec<CodeLocation>) -> Self {
        return Self {
            message: message,
            location: locations,
            stack_trace: Arc::new(Backtrace::capture()),
        };
    }

    pub fn print(&self) -> String {
        let mut output_string = String::new();
        output_string.push_str(&format!("{}\n", self.message));
        for single_location in &self.location {
            output_string.push_str(&format!("{}\n", single_location.show(Some(single_location.src_file.clone()))));
        }
        output_string.push_str(&format!("{:#?}\n", *self.stack_trace));
        return output_string;
    }

}
