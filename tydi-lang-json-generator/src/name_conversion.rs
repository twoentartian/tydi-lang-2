use std::sync::{Arc, RwLock};

use tydi_lang_parser::{tydi_memory_representation::{Variable, Port, Scope, GlobalIdentifier, Streamlet}, trait_common::GetName};

use crate::util;

pub fn get_global_variable_name(var: Arc<RwLock<Variable>>) -> String {
    let parent_scope = var.read().unwrap().get_parent_scope();
    let scope_part = match parent_scope {
        Some(scope) => {
            scope.read().unwrap().get_name()
        },
        None => {
            format!("{}", util::generate_random_str(4))
        },
    };
    let variable_part = var.read().unwrap().get_name();
    let output_name = format!("{}__{}", scope_part, variable_part);
    return remove_unaccepted_char(output_name);;
}

pub fn get_global_variable_name_with_scope<T: GetName>(var: Arc<RwLock<T>>, scope: Arc<RwLock<Scope>>) -> String {
    let scope_part = scope.read().unwrap().get_name();
    let variable_part = var.read().unwrap().get_name();
    let output_name = format!("{}__{}", scope_part, variable_part);
    return remove_unaccepted_char(output_name);
}

pub fn get_global_variable_name_with_parent_scope<T: GetName + GlobalIdentifier>(var: Arc<RwLock<T>>) -> String {
    let parent_scope = var.read().unwrap().get_parent_scope().expect("bug: no parent for this variable");
    let scope_part = parent_scope.read().unwrap().get_name();
    let variable_part = match var.read().unwrap().get_id_in_scope() {
        Some(id) => id,
        None => var.read().unwrap().get_name(),
    };
    let output_name = format!("{}__{}", scope_part, variable_part);
    return remove_unaccepted_char(output_name);
}

fn remove_unaccepted_char(src: String) -> String {
    let mut output_name = src;
    output_name = output_name.replace("!", "");
    output_name = output_name.replace(" ", "");
    output_name = output_name.replace("(", "");
    output_name = output_name.replace(")", "");
    output_name = output_name.replace("<", "");
    output_name = output_name.replace(">", "");
    return output_name;;
}