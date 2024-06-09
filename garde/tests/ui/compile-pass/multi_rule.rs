#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(prefix("test"), ascii, length(min = 10, max = 100))]
    field: &'a str,
    #[garde(inner(prefix("test"), ascii, length(min = 10, max = 100)))]
    inner: &'a [&'a str],
}

fn main() {}
