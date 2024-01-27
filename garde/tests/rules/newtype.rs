#![allow(non_camel_case_types)]

use super::util;

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct NonEmptyStr_Struct<'a> {
    #[garde(length(min = 1))]
    v: &'a str,
}

#[test]
fn newtype_struct_valid() {
    util::check_ok(&[NonEmptyStr_Struct { v: "test" }], &());
}

#[test]
fn newtype_struct_invalid() {
    util::check_fail!(&[NonEmptyStr_Struct { v: "" }], &());
}

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct NonEmptyStr_Tuple<'a>(#[garde(length(min = 1))] &'a str);

#[test]
fn newtype_tuple_valid() {
    util::check_ok(&[NonEmptyStr_Tuple("test")], &());
}

#[test]
fn newtype_tuple_invalid() {
    util::check_fail!(&[NonEmptyStr_Tuple("")], &());
}

#[derive(Debug, garde::Validate)]

struct Test<'a> {
    #[garde(dive)]
    a: NonEmptyStr_Struct<'a>,
    #[garde(dive)]
    b: NonEmptyStr_Tuple<'a>,
}

#[test]
fn newtype_fields_valid() {
    util::check_ok(
        &[Test {
            a: NonEmptyStr_Struct { v: "test" },
            b: NonEmptyStr_Tuple("test"),
        }],
        &(),
    );
}

#[test]
fn newtype_fields_invalid() {
    util::check_fail!(
        &[Test {
            a: NonEmptyStr_Struct { v: "" },
            b: NonEmptyStr_Tuple(""),
        }],
        &()
    );
}
