use std::sync::{Arc, RwLock};

use crate::tydi_memory_representation::{TypedValue, Project};
use crate::generate_get_pub;

#[derive(Clone, Debug)]
pub struct EvaluationTrace {
    pub evaluated_target_name: String,
    pub evaluated_value: TypedValue,
}

impl EvaluationTrace {
    pub fn new(evaluated_target_name: String, evaluated_value: TypedValue) -> Self {
        let output = Self {
            evaluated_target_name: evaluated_target_name,
            evaluated_value: evaluated_value,
        };
        return output;
    }
}

#[derive(Clone, Debug)]
pub struct EvaluationRecord {
    traces: Vec<EvaluationTrace>,
}

impl EvaluationRecord {
    pub fn new() -> Self {
        let output = Self {
            traces: vec![],
        };
        return output;
    }

    pub fn add_trace(&mut self, evaluated_target_name: String, evaluated_value: TypedValue) {
        self.traces.push(EvaluationTrace::new(evaluated_target_name, evaluated_value));
    }

    generate_get_pub!(traces, Vec<EvaluationTrace>, get_traces);
}


pub struct Evaluator {
    project: Arc<RwLock<Project>>,
    evaluation_record: EvaluationRecord,
}

impl Evaluator {
    pub fn new(project: Arc<RwLock<Project>>) -> Arc<RwLock<Self>> {
        let output = Self {
            project: project,
            evaluation_record: EvaluationRecord::new(),
        };
        return Arc::new(RwLock::new(output));
    }

    pub fn evaluate(&mut self, entry_implementation_name: String, entry_package: String) {
        todo!()
    }

    generate_get_pub!(evaluation_record, EvaluationRecord, get_evaluation_record);
}