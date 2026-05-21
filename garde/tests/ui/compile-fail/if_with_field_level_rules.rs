#![allow(dead_code)]

#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(skip)]
    enabled: bool,
    #[garde(if(cond = self.enabled, skip))]
    skip: &'a str,
    #[garde(if(cond = self.enabled, rename("renamed")))]
    rename: &'a str,
    #[garde(if(cond = self.enabled, adapt(adapter)))]
    adapt: &'a str,
    #[garde(if(cond = self.enabled, code("code")))]
    code: &'a str,
}

mod adapter {}

fn main() {}
