use serde::{Serialize};

use crate::deep_clone::DeepClone;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum Attribute {
    NoStrictTypeChecking,
    External,
    NoTemplateExpansion,
}

impl DeepClone for Attribute {
    fn deep_clone(&self) -> Self {
        return self.clone();
    }
}

impl std::convert::TryFrom<String> for Attribute {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "NoTypeCheck" || value == "no_type_check" {
            return Ok(Attribute::NoStrictTypeChecking);
        }
        if value == "External" || value == "external" {
            return Ok(Attribute::External);
        }
        if value == "NoTemplateExpansion" || value == "no_template_expansion" {
            return Ok(Attribute::NoTemplateExpansion);
        }

        return Err(());
    }
}