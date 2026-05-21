#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(skip)]
    enabled: bool,
    #[garde(dive, if(cond = self.enabled, inner(length(min = 1))))]
    field: &'a [&'a str],
}

fn main() {}
