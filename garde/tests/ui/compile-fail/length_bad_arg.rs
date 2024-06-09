#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 10, invalid_arg, max = 50))]
    field: &'a str,
    #[garde(length(min = 10, invalid_arg = 10, max = 50))]
    field2: &'a str,
}

fn main() {}
