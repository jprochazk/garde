#![allow(dead_code)]

const TEST: &str = "test";

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(contains("test"))]
    field: &'a str,
    #[garde(inner(contains(TEST)))]
    inner: &'a [&'a str],
}

fn main() {}
