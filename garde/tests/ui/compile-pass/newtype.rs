#![allow(dead_code)]

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct UnaryTuple<'a>(#[garde(ascii)] &'a str);

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct UnaryStruct<'a> {
    #[garde(ascii)]
    a: &'a str,
}

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct TupleWithSkippedField<'a>(#[garde(ascii)] &'a str, #[garde(skip)] &'a str);

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct StructWithSkippedField<'a> {
    #[garde(ascii)]
    a: &'a str,
    #[garde(skip)]
    b: &'a str,
}

fn main() {}
