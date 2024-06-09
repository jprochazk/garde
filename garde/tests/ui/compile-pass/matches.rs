#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(skip)]
    foo: &'a str,
    #[garde(matches(foo))]
    bar: &'a str,
}

fn main() {}
