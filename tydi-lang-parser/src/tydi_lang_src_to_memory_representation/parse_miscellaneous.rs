use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;

use crate::generate_name::generate_init_value;
use crate::tydi_memory_representation::{Scope, Attribute, CodeLocation};
use crate::tydi_parser::*;

#[allow(non_snake_case)]
pub fn parse_ATTRIBUTE(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Attribute, TydiLangError> {
    let attr = src.as_str().to_string();
    let attr = attr.replace("@", "");  // PEST 12: ATTRIBUTE = @{ "@" ~ ID }
    let result = Attribute::try_from(attr.clone());
    if result.is_err() {
        return Err(TydiLangError::new(format!("unknown attribute: {}", attr), CodeLocation::new_from_pest_rule(&src)));
    }
    return Ok(result.ok().unwrap());
}