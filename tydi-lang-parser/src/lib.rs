extern crate pest;
#[macro_use]
extern crate pest_derive;

mod tydi_parser;
mod test_tydi_parser;
mod error;
mod util;
mod generate_name;

mod deep_clone;
mod serde_serialization;

pub mod trait_common;

pub mod tydi_memory_representation;

mod tydi_lang_src_to_memory_representation;

mod evaluation;

extern crate strum;
#[macro_use]
extern crate strum_macros;


mod test_project;
