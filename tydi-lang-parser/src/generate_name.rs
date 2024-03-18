use std::sync::{Arc, RwLock, atomic::AtomicUsize};
use std::collections::BTreeMap;

use crate::trait_common::GetName;
use crate::{tydi_parser::*, util, tydi_memory_representation::{Variable, TypedValue}};

static mut GENERATE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static mut TEMPLATE_INSTANCE_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn generate_built_in_variable_name_from_span(src: &Pair<Rule>) -> String {
    let src_span = src.as_span();
    let start_pos = src_span.start_pos().pos();
    let end_pos = src_span.end_pos().pos();
    let counter;
    unsafe {
        GENERATE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        counter = GENERATE_COUNTER.load(std::sync::atomic::Ordering::SeqCst);
    }
    // format!("generated_{}_{}_{}_{}", start_pos, end_pos, util::generate_random_str(8), counter)
    format!("generated_{}_{}_{}", start_pos, end_pos, counter)
}

pub fn generate_template_instance_name(template_var: Arc<RwLock<Variable>>, template_exps: &BTreeMap<usize, TypedValue>) -> String {
    let template_var_name = template_var.read().unwrap().get_name();
    let mut arg_part = String::new();
    for index in 0..template_exps.len() {
        arg_part.push_str(&template_exps.get(&index).unwrap().get_brief_info());
    }
    let counter;
    unsafe {
        TEMPLATE_INSTANCE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        counter = TEMPLATE_INSTANCE_COUNTER.load(std::sync::atomic::Ordering::SeqCst);
    }
    format!("instance_{}_{}_{}", template_var_name, arg_part, counter)
}

pub fn generate_template_instance_name_based_on_old_name(old_name: String, template_exps: &BTreeMap<usize, TypedValue>) -> String {
    let mut arg_part = String::new();
    for index in 0..template_exps.len() {
        arg_part.push_str(&template_exps.get(&index).unwrap().get_brief_info());
    }

    format!("instance_{}_{}", old_name, arg_part)
}

pub fn generate_init_value() -> String {
    format!("???")
}