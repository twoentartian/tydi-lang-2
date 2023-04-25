use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::tydi_memory_representation::{Scope, Variable};
use crate::{tydi_parser::*, generate_name};

pub fn create_variable_from_exp(src: Pair<Rule>, scope: Arc<RwLock<Scope>>) -> Result<Arc<RwLock<Variable>>, TydiLangError> {
    let src_span = src.as_span();
    let src_start = src_span.start_pos().pos();
    let src_end = src_span.end_pos().pos();
    let var_name = generate_name::generate_built_in_variable_name(src_start, src_end);
    let var = Variable::new(var_name, Some(src.as_str().to_string()));
    {
        let mut scope_write = scope.write().unwrap();
        scope_write.add_var(var.clone())?;
    }
    return Ok(var.clone());
}