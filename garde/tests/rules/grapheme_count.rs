use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(grapheme_count(min = 10, max = 100))]
    field: String,
    #[garde(inner(grapheme_count(min = 10, max = 100)))]
    inner: &'a [String],
}

#[test]
fn grapheme_count_valid() {
    util::check_ok(
        &[
            Test {
                // 'a' = 1 grapheme
                field: "a".repeat(10),
                inner: &["a".repeat(10)],
            },
            Test {
                field: "a".repeat(100),
                inner: &["a".repeat(100)],
            },
            Test {
                // '😂' = 1 grapheme
                field: "😂".repeat(100),
                inner: &["😂".repeat(100)],
            },
            Test {
                // '👨‍👩‍👦' = 1 grapheme
                field: "👨‍👩‍👦".repeat(10),
                inner: &["👨‍👩‍👦".repeat(10)],
            },
            Test {
                field: "👨‍👩‍👦".repeat(100),
                inner: &["👨‍👩‍👦".repeat(100)],
            },
            Test {
                // '😂👨‍👩‍👦' = 2 graphemes
                field: "😂👨‍👩‍👦".repeat(5),
                inner: &["😂👨‍👩‍👦".repeat(5)],
            },
            Test {
                field: "😂👨‍👩‍👦".repeat(50),
                inner: &["😂👨‍👩‍👦".repeat(50)],
            },
        ],
        &(),
    )
}

#[test]
fn grapheme_count_invalid() {
    util::check_fail!(
        &[
            Test {
                // 'a' = 1 grapheme
                field: "a".repeat(9),
                inner: &["a".repeat(9)],
            },
            Test {
                field: "a".repeat(101),
                inner: &["a".repeat(101)],
            },
            Test {
                // '😂' = 1 grapheme
                field: "😂".repeat(101),
                inner: &["😂".repeat(101)],
            },
            Test {
                // '👨‍👩‍👦' = 1 grapheme
                field: "👨‍👩‍👦".repeat(9),
                inner: &["👨‍👩‍👦".repeat(9)],
            },
            Test {
                field: "👨‍👩‍👦".repeat(101),
                inner: &["👨‍👩‍👦".repeat(101)],
            },
            Test {
                // '😂👨‍👩‍👦' = 2 graphemes
                field: "😂👨‍👩‍👦".repeat(4),
                inner: &["😂👨‍👩‍👦".repeat(4)],
            },
            Test {
                field: "😂👨‍👩‍👦".repeat(51),
                inner: &["😂👨‍👩‍👦".repeat(51)],
            },
        ],
        &()
    )
}

#[derive(Debug, garde::Validate)]
struct Exact<'a> {
    #[garde(grapheme_count(min = 5, max = 5))]
    field: String,
    #[garde(inner(grapheme_count(min = 5, max = 5)))]
    inner: &'a [String],
}

#[test]
fn exact_grapheme_count_valid() {
    util::check_ok(
        &[
            Exact {
                field: "a".repeat(5),
                inner: &["a".repeat(5)],
            },
            Exact {
                field: "👨‍👩‍👦".repeat(5),
                inner: &["👨‍👩‍👦".repeat(5)],
            },
            // '你हूँאਲੋ😂' = 5 graphemes
            Exact {
                field: "你हूँאਲੋ😂".into(),
                inner: &["你हूँאਲੋ😂".into()],
            },
        ],
        &(),
    )
}

#[test]
fn exact_grapheme_count_invalid() {
    util::check_fail!(
        &[
            Exact {
                field: "".into(),
                inner: &["".into()],
            },
            Exact {
                field: "a".into(),
                inner: &["a".into()],
            },
            Exact {
                field: "a".repeat(3),
                inner: &["a".repeat(3)],
            },
            Exact {
                field: "😂".repeat(4),
                inner: &["😂".repeat(4)],
            },
            // '你हूँאਲੋ😂ア' = 6 graphemes
            Exact {
                field: "你हूँאਲੋ😂ア".into(),
                inner: &["你हूँאਲੋ😂ア".into()],
            },
        ],
        &()
    )
}
