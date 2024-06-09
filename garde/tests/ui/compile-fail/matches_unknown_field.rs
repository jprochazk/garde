#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(matches(foo))]
    bar: &'a str,
}

fn main() {}
