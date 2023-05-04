use serde::{Serialize};

#[derive(Clone, Debug, Serialize)]
pub enum Attribute {
    NoStrictTypeChecking,

}

impl std::convert::TryFrom<String> for Attribute {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "NoTypeCheck" {
            return Ok(Attribute::NoStrictTypeChecking);
        }
        return Err(());
    }
}