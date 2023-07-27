#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(skip)]
    field: &'a str,
}

fn main() {}
