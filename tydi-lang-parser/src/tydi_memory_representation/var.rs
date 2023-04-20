use std::sync::{Arc, RwLock};

use serde::{Serialize, Deserialize};

use crate::tydi_memory_representation::TypedValue;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EvaluationStatus {
    NotEvaluated,
    EvaluationCount(u32),
    Evaluated,
}

#[derive(Clone, Debug, Serialize)]
pub struct Variable {
    name: String,

    exp: Option<String>,

    evaluated: EvaluationStatus,

    value: TypedValue,
}