#![allow(dead_code)]

mod reexport {
    pub use garde::*;
}

// Basic crate attribute usage
#[derive(garde::Validate)]
#[garde(crate = reexport)]
struct Foo<'a> {
    #[garde(length(min = 1))]
    field: &'a str,
}

// crate attribute combined with other attributes
#[derive(garde::Validate)]
#[garde(crate = reexport)]
#[garde(allow_unvalidated)]
struct Bar<'a> {
    field: &'a str,
}

// crate attribute with context
#[derive(garde::Validate)]
#[garde(crate = reexport)]
#[garde(context(()))]
struct Baz<'a> {
    #[garde(ascii)]
    field: &'a str,
}

fn main() {}
