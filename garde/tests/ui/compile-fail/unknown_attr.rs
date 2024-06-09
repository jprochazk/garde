#![allow(dead_code)]

#[derive(garde::Validate)]
#[garde(unknown_attr)]
struct Test {}

fn main() {}
