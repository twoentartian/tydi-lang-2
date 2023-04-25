#[allow(unused_imports)]
pub use pest::Parser;

#[allow(unused_imports)]
pub use pest::iterators::{Pair, Pairs};

#[derive(Parser)]
#[grammar = "./tydi_lang_grammar_exp.pest"]
pub struct TydiLangSrc;

