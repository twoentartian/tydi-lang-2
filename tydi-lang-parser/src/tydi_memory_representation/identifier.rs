use std::sync::{RwLock, Arc};
use std::collections::BTreeMap;

use serde::{Serialize};

use crate::generate_get_pub;

#[derive(Clone, Debug, Serialize)]
pub enum IdentifierType {
    Unknown,

    FunctionExp(/*function_args : */BTreeMap<usize, String>),
    IndexExp(/*index : */String),
    IdentifierExp,
}


#[derive(Clone, Debug, Serialize)]
pub struct Identifier {
    pub id: String,
    pub id_type: IdentifierType,
    pub template_args: BTreeMap<usize, String>,
}

impl Identifier {
    pub fn new(id: String, id_type: IdentifierType, template_args: BTreeMap<usize, String>) -> Arc<RwLock<Self>> {
        let output = Self {
            id: id,
            id_type: id_type,
            template_args: template_args,
        };
        return Arc::new(RwLock::new(output));
    }

    generate_get_pub!(id, String, get_id);
}