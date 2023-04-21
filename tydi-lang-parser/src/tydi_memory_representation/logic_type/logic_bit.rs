use std::sync::{Arc, RwLock};

use serde::{Serialize};

use crate::tydi_memory_representation::{Variable, TypeIndication};

use crate::name_trait::GetName;

#[derive(Clone, Debug, Serialize)]
pub struct LogicBit {
    name: String,

    #[serde(with = "crate::serde_arc_rwlock_inner")]
    bit_width: Arc<RwLock<Variable>>,

}

impl GetName for LogicBit {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl LogicBit {
    fn new(name: String, exp_bit_width: String) -> Arc<RwLock<LogicBit>> {
        let mut output = LogicBit {
            name: name.clone(),
            bit_width: Variable::new(format!("bitwidth_{name}"), Some(exp_bit_width)),
        };
        {
            let mut output_bit_width_write = output.bit_width.write().unwrap();
            output_bit_width_write.type_indication = TypeIndication::Int;
        }
        return Arc::new(RwLock::new(output));
    }
}