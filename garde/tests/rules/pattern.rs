use once_cell::sync::Lazy;
use regex::Regex;

use super::util;

mod sub {
    use super::*;
    pub static LAZY_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^abcd|efgh$").unwrap());
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(pattern(r"^abcd|efgh$"))]
    field: &'a str,

    #[garde(pattern(sub::LAZY_RE))]
    field_path: &'a str,

    #[garde(pattern(create_regex()))]
    field_call: &'a str,

    #[garde(inner(pattern(r"^abcd|efgh$")))]
    inner: &'a [&'a str],
}

fn create_regex() -> Regex {
    Regex::new(r"^abcd|efgh$").unwrap()
}

#[test]
fn pattern_valid() {
    util::check_ok(
        &[
            Test {
                field: "abcd",
                field_path: "abcd",
                field_call: "abcd",
                inner: &["abcd"],
            },
            Test {
                field: "efgh",
                field_path: "efgh",
                field_call: "efgh",
                inner: &["efgh"],
            },
        ],
        &(),
    )
}

#[test]
fn pattern_invalid() {
    util::check_fail!(
        &[
            Test {
                field: "dcba",
                field_path: "dcba",
                field_call: "dcba",
                inner: &["dcba"]
            },
            Test {
                field: "hgfe",
                field_path: "hgfe",
                field_call: "hgfe",
                inner: &["hgfe"]
            }
        ],
        &()
    )
}
