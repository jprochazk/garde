use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(char_count(min = 10, max = 100))]
    field: &'a str,
    #[garde(inner(char_count(min = 10, max = 100)))]
    inner: &'a [&'a str],
}

#[test]
fn char_count_valid() {
    util::check_ok(&[
        Test {
            // 'a' * 10
            field: "aaaaaaaaaa",
            inner: &["aaaaaaaaaa"],
        },
        Test {
            // 'a' * 100
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            inner: &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
        },
        Test {
            // "😂" = 1 char
            field: &"😂".repeat(100),
            inner: &[&"😂".repeat(100)],
        },
        Test {
            // "👨‍👩‍👦" = 5 chars
            // "👨‍👩‍👦" * 2 = 10 chars
            field: &"👨‍👩‍👦".repeat(2),
            inner: &[&"👨‍👩‍👦".repeat(2)],
        },
        Test {
            // "👨‍👩‍👦" = 5 chars
            // "👨‍👩‍👦" * 20 = 100 chars
            field: &"👨‍👩‍👦".repeat(20),
            inner: &[&"👨‍👩‍👦".repeat(20)],
        },
    ], &())
}

#[test]
fn char_count_invalid() {
    util::check_fail!(&[
        Test {
            // 'a' * 9
            field: "aaaaaaaaa",
            inner: &["aaaaaaaaa"],
        },
        Test {
            // 'a' * 101
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            inner: &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
        },
        Test {
            // "😂" = 1 char
            field: &"😂".repeat(101),
            inner: &[&"😂".repeat(101)],
        },
        Test {
            // "👨‍👩‍👦" = 5 chars
            field: "👨‍👩‍👦",
            inner: &["👨‍👩‍👦"],
        },
        Test {
            // "👨‍👩‍👦" = 5 chars
            // "👨‍👩‍👦" * 21 = 105 chars
            field: &"👨‍👩‍👦".repeat(21),
            inner: &[&"👨‍👩‍👦".repeat(21)],
        },
    ], &())
}

#[derive(Debug, garde::Validate)]
struct Exact<'a> {
    #[garde(char_count(min = 5, max = 5))]
    field: &'a str,
    #[garde(inner(char_count(min = 5, max = 5)))]
    inner: &'a [&'a str],
}

#[test]
fn exact_char_count_valid() {
    util::check_ok(
        &[
            Exact {
                // 'a' * 5
                field: "aaaaa",
                inner: &["aaaaa"],
            },
            Exact {
                field: "👨‍👩‍👦",
                inner: &["👨‍👩‍👦"],
            },
        ],
        &(),
    )
}

#[test]
fn exact_char_count_invalid() {
    util::check_fail!(
        &[
            Exact {
                field: "",
                inner: &[""],
            },
            Exact {
                // 'a' * 1
                field: "a",
                inner: &["a"],
            },
            Exact {
                // 'a' * 3
                field: "aaa",
                inner: &["aaa"],
            },
            Exact {
                field: "😂",
                inner: &["😂"],
            },
            Exact {
                field: "👨‍👩‍👦👨‍👩‍👦",
                inner: &["👨‍👩‍👦👨‍👩‍👦"],
            },
        ],
        &()
    )
}
