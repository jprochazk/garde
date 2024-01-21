use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 10, max = 100))]
    field: &'a str,
    #[garde(inner(length(min = 10, max = 100)))]
    inner: &'a [&'a str],
}

#[test]
fn length_valid() {
    util::check_ok(&[
        Test {
            // 'a' * 10
            field: "aaaaaaaaaa",
            inner: &["aaaaaaaaaa"]
        },
        Test {
            // 'a' * 100
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            inner: &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]
        },
    ], &())
}

#[test]
fn length_invalid() {
    util::check_fail!(&[
        Test {
            // 'a' * 9
            field: "aaaaaaaaa",
            inner: &["aaaaaaaaa"]
        },
        Test {
            // 'a' * 101
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            inner: &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]
        },
    ], &())
}

#[derive(Debug, garde::Validate)]
struct Exact<'a> {
    #[garde(length(min = 2, max = 2))]
    field: &'a str,
    #[garde(inner(length(min = 2, max = 2)))]
    inner: &'a [&'a str],
}

#[test]
fn exact_length_valid() {
    util::check_ok(
        &[Exact {
            // 'a' * 2
            field: "aa",
            inner: &["aa"],
        }],
        &(),
    )
}

#[test]
fn exact_length_invalid() {
    util::check_fail!(
        &[
            Exact {
                field: "",
                inner: &[""]
            },
            Exact {
                // 'a' * 1
                field: "a",
                inner: &["a"]
            },
            Exact {
                // 'a' * 3
                field: "aaa",
                inner: &["aaa"]
            },
        ],
        &()
    )
}

#[derive(Debug, garde::Validate)]
struct SpecialLengthTest<'a> {
    #[garde(length(simple, max = 1))]
    simple: &'a str,
    #[garde(length(bytes, max = 1))]
    bytes: &'a str,
    #[garde(length(chars, max = 1))]
    chars: &'a str,
    #[garde(length(graphemes, max = 1))]
    graphemes: &'a str,
    #[garde(length(utf16, max = 1))]
    utf16: &'a str,
}

#[test]
fn char_length_valid() {
    util::check_ok(
        &[SpecialLengthTest {
            simple: "a",
            bytes: "a",
            chars: "á",
            graphemes: "á",
            utf16: "á",
        }],
        &(),
    )
}

#[test]
fn char_length_invalid() {
    util::check_fail!(
        &[SpecialLengthTest {
            simple: "ab",    // 2 bytes
            bytes: "ab",     // 2 bytes
            chars: "y̆",      // 2 USVs
            graphemes: "áá", // 2 graphemes
            utf16: "😂"      // 2 units
        }],
        &()
    )
}
