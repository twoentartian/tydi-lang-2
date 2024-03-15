use std::sync::{Arc, RwLock};

use crate::error::TydiLangError;
use crate::tydi_memory_representation::{Scope, Project, GetScope};
use crate::evaluation::{Evaluator, evaluate_var, EvaluationTrace};

/*
pub const STD_LIB_PACKAGE_NAME: Option<String> = None; // works for project which only has one package
pub const STD_LIB_PACKAGE_NAME: Option<String> = Some(xxx); // works for project which contains more than one packages
 */
pub const STD_LIB_PACKAGE_NAME: &str = "std";

/*
impl void_i<type_in: type> of void_s<type_in> @External {}
 */
pub const STD_VOID_IMPL_NAME: &str = "void_i";

/*
impl void_i<type_in: type> of void_s<type_in> @External {}
*/
pub const STD_DUPLICATOR_IMPL_NAME: &str = "duplicator_i";

// automatically insert duplicators and voiders
pub fn sugaring_connections(project: Arc<RwLock<Project>>, evaluator: Arc<RwLock<Evaluator>>) -> Result<(), TydiLangError> {
    
    //find the std implementations
    todo!()

}
