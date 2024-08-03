#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 10, max = 100))]
    field: &'a str,
    #[garde(length(equal = 10))]
    field2: &'a str,
    #[garde(length(min = 10, max = 10))]
    field3: &'a str,
    #[garde(inner(length(min = 10, max = 100)))]
    inner: &'a [&'a str],
    #[garde(inner(length(equal = 10)))]
    inner2: &'a [&'a str],
    #[garde(inner(length(min = 10, max = 10)))]
    inner3: &'a [&'a str],

    #[garde(length(simple, equal = 1))]
    simple: &'a str,
    #[garde(length(bytes, equal = 1))]
    bytes: &'a str,
    #[garde(length(chars, equal = 1))]
    chars: &'a str,
    #[garde(length(graphemes, equal = 1))]
    graphemes: &'a str,
    #[garde(length(utf16, equal = 1))]
    utf16: &'a str,
}

fn main() {}
