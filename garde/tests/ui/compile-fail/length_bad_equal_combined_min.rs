#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 1, equal = 10))]
    min_equal: &'a str,
}

fn main() {}
