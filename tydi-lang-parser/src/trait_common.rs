use std::sync::{Arc, RwLock};

use crate::tydi_memory_representation::Variable;

pub trait GetName {
    fn get_name(&self) -> String;
}

pub trait AccessProperty {
    fn access_porperty(&self, property_name: &String) -> Option<Arc<RwLock<Variable>>>;
}

pub trait HasDocument {
    fn set_document(&mut self, docuemnt: Option<String>);

    fn get_document(&self) -> Option<String>;
}

pub trait NewPlaceHolder {
    fn new_place_holder() -> Self;
}