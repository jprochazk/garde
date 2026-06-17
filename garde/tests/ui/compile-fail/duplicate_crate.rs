#![allow(dead_code)]

mod reexport {
    pub use valuable::*;
}

#[derive(garde::Validate)]
#[garde(crate = reexport)]
#[garde(crate = reexport)]
struct Foo<'a> {
    field: &'a str,
}

fn main() {}
