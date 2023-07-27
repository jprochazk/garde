#![allow(dead_code)]

#[derive(garde::Validate)]
#[garde(allow_unvalidated)]
struct Foo<'a> {
    field: &'a str,
}

// should also work with `context`
#[derive(garde::Validate)]
#[garde(allow_unvalidated)]
#[garde(context(()))]
struct Bar<'a> {
    field: &'a str,
}

fn main() {}
