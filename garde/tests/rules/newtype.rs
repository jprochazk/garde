#![allow(non_camel_case_types)]

use super::util;

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct NonEmptyStr_Struct<'a> {
    #[garde(length(min = 1))]
    v: &'a str,
}

#[derive(Debug, garde::Validate)]
#[garde(transparent)]
struct NonEmptyStr_Tuple<'a>(#[garde(length(min = 1))] &'a str);

#[derive(Debug, garde::Validate)]

struct Test<'a> {
    #[garde(dive)]
    a: NonEmptyStr_Struct<'a>,
    #[garde(dive)]
    b: NonEmptyStr_Tuple<'a>,
}

#[test]
fn newtype_valid() {
    util::check_ok(
        &[Test {
            a: NonEmptyStr_Struct { v: "test" },
            b: NonEmptyStr_Tuple("test"),
        }],
        &(),
    );
}

#[test]
fn newtype_invalid() {
    util::check_fail!(
        &[Test {
            a: NonEmptyStr_Struct { v: "" },
            b: NonEmptyStr_Tuple(""),
        }],
        &()
    );
}
