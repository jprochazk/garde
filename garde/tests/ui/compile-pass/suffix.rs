#![allow(dead_code)]

const TEST: &str = "test";

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(suffix(TEST))]
    field: &'a str,
    #[garde(inner(suffix("test")))]
    inner: &'a [&'a str],
}

fn main() {}
