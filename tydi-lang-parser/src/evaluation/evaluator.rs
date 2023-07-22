use std::sync::{Arc, RwLock};

use crate::tydi_memory_representation::{TypedValue, Project};
use crate::generate_get_pub;

#[derive(Clone, Debug, PartialEq)]
pub enum EvaluationTraceType {
    Unknwon,
    BeginRegion(String),
    EndRegion(String),
    StartEvaluation,
    FinishEvaluation,
}

#[derive(Clone, Debug)]
pub struct EvaluationTrace {
    pub evaluated_target_name: String,
    pub evaluated_value: Option<TypedValue>,
    pub trace_type: EvaluationTraceType,
    pub deepth: usize,
}

impl EvaluationTrace {
    pub fn new(evaluated_target_name: String, evaluated_value: Option<TypedValue>, trace_type: EvaluationTraceType, deepth: usize) -> Self {
        let output = Self {
            evaluated_target_name: evaluated_target_name,
            evaluated_value: evaluated_value,
            trace_type: trace_type,
            deepth: deepth,
        };
        return output;
    }

    pub fn new_region_begin(region_name: String) -> Self {
        let output = Self {
            evaluated_target_name: String::from("???"),
            evaluated_value: None,
            trace_type: EvaluationTraceType::BeginRegion(region_name),
            deepth: 0,
        };
        return output;
    }

    pub fn new_region_end(region_name: String) -> Self {
        let output = Self {
            evaluated_target_name: String::from("???"),
            evaluated_value: None,
            trace_type: EvaluationTraceType::EndRegion(region_name),
            deepth: 0,
        };
        return output;
    }

    pub fn print_line(&self) -> String {
        let mut output = String::new();
        for _ in 0..self.deepth {output.push_str(" ");}

        match &self.trace_type {
            EvaluationTraceType::Unknwon => unreachable!(),
            EvaluationTraceType::BeginRegion(region_name) => {
                output.push_str(&format!("region [{}] begins\n", region_name));
            },
            EvaluationTraceType::EndRegion(region_name) => {
                output.push_str(&format!("region [{}] ends\n\n", region_name));
            },
            EvaluationTraceType::StartEvaluation | EvaluationTraceType::FinishEvaluation => {
                match &self.evaluated_value {
                    Some(evaluated_value) => output.push_str(&format!("{} --> {} ({:?})\n", &self.evaluated_target_name, evaluated_value.get_brief_info(), self.trace_type)),
                    None => output.push_str(&format!("{} --> ??? ({:?})\n", &self.evaluated_target_name, self.trace_type)),
                }
            },
        }

        return output;
    }
}

#[derive(Clone, Debug)]
pub struct EvaluationRecord {
    traces: Vec<EvaluationTrace>,
    current_deepth: usize,
}

impl EvaluationRecord {
    pub fn new() -> Self {
        let output = Self {
            traces: vec![],
            current_deepth : 0,
        };
        return output;
    }

    pub fn add_evaluation_trace(&mut self, evaluated_target_name: String, evaluated_value: Option<TypedValue>, trace_type: EvaluationTraceType) {
        self.traces.push(EvaluationTrace::new(evaluated_target_name, evaluated_value, trace_type, self.current_deepth));
    }

    pub fn add_trace(&mut self, trace: EvaluationTrace) {
        self.traces.push(trace);
    }

    pub fn increase_deepth(&mut self){
        self.current_deepth += 1;
    }

    pub fn decrease_deepth(&mut self){
        self.current_deepth -= 1;
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

    pub fn add_evaluation_trace(&mut self, evaluated_target_name: String, evaluated_value: Option<TypedValue>, trace_type: EvaluationTraceType) {
        self.evaluation_record.add_evaluation_trace(evaluated_target_name, evaluated_value, trace_type);
    }

    pub fn add_trace(&mut self, trace: EvaluationTrace) {
        self.evaluation_record.add_trace(trace);
    }

    pub fn increase_deepth(&mut self){
        self.evaluation_record.increase_deepth();
    }

    pub fn decrease_deepth(&mut self){
        self.evaluation_record.decrease_deepth();
    }

    pub fn print_evaluation_record(&self) -> String {
        let evaluation_record = &self.evaluation_record.traces;
        let mut output = String::new();
        for single_trace in evaluation_record {
            output.push_str(&single_trace.print_line());
        }
        return output;
    }

    generate_get_pub!(evaluation_record, EvaluationRecord, get_evaluation_record);
    generate_get_pub!(project, Arc<RwLock<Project>>, get_project);
}