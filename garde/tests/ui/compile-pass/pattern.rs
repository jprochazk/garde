#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(pattern(r"a|b"))]
    field: &'a str,
    #[garde(inner(pattern(r"a|b")))]
    inner: &'a [&'a str],
}

fn main() {}
