extern crate pest;
#[macro_use]
extern crate pest_derive;


mod tydi_parser;
mod test_tydi_parser;

mod tydi_memory_representation;


#[test]
fn test(){
    tydi_memory_representation::hello_world();
}