#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(ascii, unknown_rule)]
    field: &'a str,
}

fn main() {}
