#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 1, equal = 10))]
    field: &'a str,
}

fn main() {}
