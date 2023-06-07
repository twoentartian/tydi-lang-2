use std::sync::{RwLock, Arc};
use std::collections::BTreeMap;

use serde::{Serialize};

use crate::deep_clone::DeepClone;
use crate::generate_get_pub;

#[derive(Clone, Debug, Serialize)]
pub enum IdentifierType {
    Unknown,

    FunctionExp(/*function_args : */BTreeMap<usize, String>),
    IndexExp(/*index : */String),
    IdentifierExp,
}

impl DeepClone for IdentifierType {
    fn deep_clone(&self) -> Self {
        let output = match self {
            IdentifierType::Unknown => IdentifierType::Unknown,
            IdentifierType::FunctionExp(v) => IdentifierType::FunctionExp(v.deep_clone()),
            IdentifierType::IndexExp(v) => IdentifierType::IndexExp(v.deep_clone()),
            IdentifierType::IdentifierExp => IdentifierType::IdentifierExp,
        };
        return output;
    }
}


#[derive(Clone, Debug, Serialize)]
pub struct Identifier {
    id: String,
    id_type: IdentifierType,
    template_args: BTreeMap<usize, String>,
}

impl DeepClone for Identifier {
    fn deep_clone(&self) -> Self {
        let output = Self {
            id: self.id.deep_clone(),
            id_type: self.id_type.deep_clone(),
            template_args: self.template_args.deep_clone(),
        };
        return output;
    }
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

    pub fn get_brief_info(&self) -> String {
        match &self.id_type {
            IdentifierType::Unknown => {
                return format!("unknown");
            },
            IdentifierType::FunctionExp(arg_exp) => {
                return format!("{} <{}> ({})", self.id, self.template_args.values().map(|v| v.to_string()).collect::<Vec<_>>().join(","), arg_exp.values().map(|v| v.to_string()).collect::<Vec<_>>().join(","));
            },
            IdentifierType::IndexExp(index_exp) => {
                return format!("{} <{}> [{}]", self.id, self.template_args.values().map(|v| v.to_string()).collect::<Vec<_>>().join(","), index_exp);
            },
            IdentifierType::IdentifierExp => {
                return format!("{} <{}>", self.id, self.template_args.values().map(|v| v.to_string()).collect::<Vec<_>>().join(","));
            }
        }
    }

    generate_get_pub!(id, String, get_id);
    generate_get_pub!(id_type, IdentifierType, get_id_type);
}