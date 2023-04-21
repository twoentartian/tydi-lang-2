use std::sync::{Arc, RwLock};

use serde::{Serialize};

pub mod logic_bit;
pub(in super) use logic_bit::*;

pub enum LogicType {
    LogicNullType,
    LogicBitType(Arc<RwLock<LogicBit>>),

}