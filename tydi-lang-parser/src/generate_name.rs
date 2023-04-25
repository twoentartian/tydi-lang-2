use std::sync::{Arc, RwLock, atomic::AtomicUsize};

use crate::{tydi_parser::*, util};

static mut generate_counter: AtomicUsize = AtomicUsize::new(0);

pub fn generate_built_in_variable_name(start_pos: usize, end_pos: usize) -> String {
    let counter;
    unsafe {
        generate_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        counter = generate_counter.load(std::sync::atomic::Ordering::SeqCst);
    }
    format!("!generated_{}_{}_{}_{}", start_pos, end_pos, util::generate_random_str(8), counter)
}

pub fn generate_built_in_variable_name_from_span(src: &Pair<Rule>) -> String {
    let src_span = src.as_span();
    let start_pos = src_span.start_pos().pos();
    let end_pos = src_span.end_pos().pos();
    let counter;
    unsafe {
        generate_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        counter = generate_counter.load(std::sync::atomic::Ordering::SeqCst);
    }
    format!("!generated_{}_{}_{}_{}", start_pos, end_pos, util::generate_random_str(8), counter)
}

pub fn generate_init_value() -> String {
    format!("???")
}