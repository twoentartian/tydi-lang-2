#[allow(unused_imports)]
pub use pest::Parser;

#[derive(Parser)]
#[grammar = "./tydi_lang_grammar_exp.pest"]
pub struct TydiLangSrc;

