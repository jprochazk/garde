#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(phone_number)]
    field: &'a str,
    #[garde(inner(phone_number))]
    inner: &'a [&'a str],
}

fn main() {}
