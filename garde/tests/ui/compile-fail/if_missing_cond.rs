#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(if(ascii, length(min = 1)))]
    field: &'a str,
}

fn main() {}
