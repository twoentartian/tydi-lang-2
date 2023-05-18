use std::sync::{Arc, RwLock};

use std::backtrace::Backtrace;

use crate::tydi_memory_representation::CodeLocation;

#[derive(Clone, Debug)]
pub struct TydiLangError {
    pub message: String,
    pub location: CodeLocation,
    pub stack_trace: Arc<Backtrace>,
}

impl TydiLangError {
    pub fn new(message: String, location: CodeLocation) -> Self {
        return Self {
            message: message,
            location: location,
            stack_trace: Arc::new(Backtrace::capture()),
        };
    }

    pub fn print(&self, src_ptr: Option<Arc<RwLock<String>>>) -> String {
        format!("{}\n{}\n{:?}\n", self.message, self.location.show(src_ptr), self.stack_trace)
    }

}
