use crate::tydi_memory_representation::CodeLocation;

#[derive(Clone, Debug)]
pub struct TydiLangError {
    pub message: String,
    pub location: CodeLocation,

}

impl TydiLangError {
    
}