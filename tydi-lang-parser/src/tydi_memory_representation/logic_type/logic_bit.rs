use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::{generate_access_pub, generate_get_pub, generate_set_pub};
use crate::tydi_memory_representation::{Variable, TypeIndication, CodeLocation, TraitCodeLocationAccess};

use crate::trait_common::{GetName, AccessProperty};

const BITWIDTH_VAR_NAME: &str = "width";

pub const AVAILABLE_PROPERTIES: [&'static str; 1] = [BITWIDTH_VAR_NAME];

#[derive(Clone, Debug, Serialize)]
pub struct LogicBit {
    name: String,

    #[serde(with = "crate::serde_serialization::serialize_variable_value_only")]
    bit_width: Arc<RwLock<Variable>>,

    location_define: CodeLocation,
}

impl GetName for LogicBit {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl TraitCodeLocationAccess for LogicBit {
    fn set_code_location(& mut self, loc: CodeLocation) {
        self.location_define = loc;
    }

    fn get_code_location(&self) -> CodeLocation {
        return self.location_define.clone();
    }
}

impl AccessProperty for LogicBit {
    fn access_porperty(&self, property_name: &String) -> Option<Arc<RwLock<Variable>>> {
        if property_name == BITWIDTH_VAR_NAME {
            return Some(self.bit_width.clone());
        }
        return None;
    }
}

impl LogicBit {
    pub fn new(name: String, exp_bit_width: Option<String>) -> Arc<RwLock<LogicBit>> {
        let output = LogicBit {
            name: name.clone(),
            bit_width: Variable::new(format!("{}_{name}", BITWIDTH_VAR_NAME), exp_bit_width),
            location_define: CodeLocation::new_unknown(),
        };
        {
            let mut output_bit_width_write = output.bit_width.write().unwrap();
            output_bit_width_write.set_type_indication(TypeIndication::Int);
        }
        return Arc::new(RwLock::new(output));
    }

    generate_access_pub!(bit_width, Arc<RwLock<Variable>>, get_bit_width, set_bit_width);

}
