#![allow(dead_code)]

static STR: &str = r"a|b";

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(pattern(STR))]
    field: &'a str,
}

fn main() {}
