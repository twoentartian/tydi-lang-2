use serde::{Serialize};

#[derive(Clone, Debug, Serialize)]
pub enum Attribute {
    NoStrictTypeChecking,
    External,
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
        return Err(());
    }
}