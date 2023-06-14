use std::sync::{Arc, RwLock};

use tydi_lang_parser::{tydi_memory_representation::Variable, trait_common::GetName};

use crate::util;

pub fn get_global_variable_name(var: Arc<RwLock<Variable>>) -> String {
    let parent_scope = var.read().unwrap().get_parent_scope();
    let scope_part = match parent_scope {
        Some(scope) => {
            scope.read().unwrap().get_name()
        },
        None => {
            format!("_{}_", util::generate_random_str(4))
        },
    };
    let variable_part = var.read().unwrap().get_name();
    let mut output_name = format!("{}__{}", scope_part, variable_part);
    output_name = output_name.replace("!", "");
    return output_name;
}