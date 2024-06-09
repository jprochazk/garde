#![allow(dead_code)]

#[derive(Debug, garde::Validate)]
struct Inner<'a> {
    #[garde(length(min = 1))]
    field: &'a str,
    #[garde(inner(length(min = 1)))]
    inner: &'a [&'a str],
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(dive, length(min = 1))]
    field: Vec<Inner<'a>>,
    #[garde(length(min = 1), inner(length(min = 1)))]
    inner: Vec<&'a str>,
}

fn main() {}
