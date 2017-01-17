#![feature(proc_macro, attr_literals)]

#[macro_use] extern crate validator_derive;
extern crate validator;
use validator::Validate;

#[derive(Validate)]
//~^ ERROR: custom derive attribute panicked
//~^^ HELP: Invalid schema level validation: `function` is required
#[validate(schema())]
struct Test {
    s: i32,
}

fn hey(_: &Test) -> Option<(String, String)> {
    None
}


fn main() {}