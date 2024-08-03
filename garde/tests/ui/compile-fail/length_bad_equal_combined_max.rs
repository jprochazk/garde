#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(max = 1, equal = 10))]
    max_equal: &'a str,
}

fn main() {}
