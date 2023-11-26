#![allow(dead_code)]

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct EmptyTuple();

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct EmptyStruct {}

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct NonUnaryTuple<'a>(#[garde(ascii)] &'a str, #[garde(ascii)] &'a str);

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct NonUnaryStruct<'a> {
    #[garde(ascii)]
    a: &'a str,
    #[garde(ascii)]
    b: &'a str,
}

fn main() {}
