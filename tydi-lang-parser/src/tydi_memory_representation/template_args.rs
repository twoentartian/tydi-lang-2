use serde::{Serialize};

use crate::tydi_memory_representation::{CodeLocation, TypeIndication};
use crate::trait_common::GetName;

use crate::{generate_name, generate_get, generate_set, generate_access, generate_access_pub, generate_set_pub, generate_get_pub};

use super::TraitCodeLocationAccess;

#[derive(Clone, Debug, Serialize)]
pub struct TemplateArg {
    name: String,

    is_array: bool,
    type_indication: TypeIndication,

    declare_location: CodeLocation,
}

impl GetName for TemplateArg {
    generate_get!(name, String, get_name);
}

impl TraitCodeLocationAccess for TemplateArg {
    generate_access!(declare_location, CodeLocation, get_code_location, set_code_location);
}

impl TemplateArg {
    pub fn new(name: String, type_indication: TypeIndication) -> Self {
        return Self {
            name: name,
            is_array: false,
            type_indication: type_indication,
            declare_location: CodeLocation::new_unknown(),
        };
    }

    pub fn new_place_holder() -> Self {
        return Self {
            name: generate_name::generate_init_value(),
            is_array: false,
            type_indication: TypeIndication::Unknown,
            declare_location: CodeLocation::new_unknown(),
        };
    }

    generate_set_pub!(name, String, set_name);
    generate_access_pub!(is_array, bool, get_is_array, set_is_array);
    generate_access_pub!(type_indication, TypeIndication, get_type_indication, set_type_indication);
}
