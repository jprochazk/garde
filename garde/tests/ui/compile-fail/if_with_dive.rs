#![allow(dead_code)]

#[derive(garde::Validate)]
struct Child {
    #[garde(ascii)]
    value: String,
}

#[derive(garde::Validate)]
struct Test {
    #[garde(skip)]
    enabled: bool,
    #[garde(if(cond = self.enabled, dive))]
    child: Child,
}

fn main() {}
