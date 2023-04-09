use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 10, max = 100))]
    field: &'a str,
}

#[test]
fn length_valid() {
    util::check_ok(&[
        Test {
            // 'a' * 10
            field: "aaaaaaaaaa",
        },
        Test {
            // 'a' * 100
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        },
    ], &())
}

#[test]
fn length_invalid() {
    util::check_fail!(&[
        Test {
            // 'a' * 9
            field: "aaaaaaaaa",
        },
        Test {
            // 'a' * 101
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        },
    ], &())
}

#[derive(Debug, garde::Validate)]
struct Exact<'a> {
    #[garde(length(min = 2, max = 2))]
    field: &'a str,
}

#[test]
fn exact_length_valid() {
    util::check_ok(
        &[Exact {
            // 'a' * 2
            field: "aa",
        }],
        &(),
    )
}

#[test]
fn exact_length_invalid() {
    util::check_fail!(
        &[
            Exact { field: "" },
            Exact {
                // 'a' * 1
                field: "a",
            },
            Exact {
                // 'a' * 3
                field: "aaa",
            },
        ],
        &()
    )
}
