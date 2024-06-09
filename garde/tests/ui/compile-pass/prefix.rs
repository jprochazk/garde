#![allow(dead_code)]

const TEST: &str = "test";

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(prefix("test"))]
    field: &'a str,
    #[garde(inner(prefix(TEST)))]
    inner: &'a [&'a str],
}

fn main() {}
