#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(skip)]
    enabled: bool,
    #[garde(if(cond = self.enabled, ascii, ascii))]
    field: &'a str,
}

fn main() {}
