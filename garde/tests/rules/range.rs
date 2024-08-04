use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(range(min = 10, max = 100))]
    field: u64,
    #[garde(range(min = 0, max = self.field))]
    refers_to_field: u64,
    #[garde(inner(range(min = 10, max = 100)))]
    inner: &'a [u64],
    #[garde(range(min = 0., max = 100.))]
    float_field: f32,
}

#[test]
fn range_valid() {
    util::check_ok(
        &[Test {
            field: 50,
            refers_to_field: 10,
            inner: &[50],
            float_field: 10.,
        }],
        &(),
    )
}

#[test]
fn range_invalid() {
    util::check_fail!(
        &[
            Test {
                field: 9,
                refers_to_field: 10,
                inner: &[9],
                float_field: -12.
            },
            Test {
                field: 101,
                refers_to_field: 200,
                inner: &[101],
                float_field: 1204.
            }
        ],
        &()
    )
}

#[derive(Debug, garde::Validate)]
struct Exact<'a> {
    #[garde(range(equal = 2))]
    field: u64,
    #[garde(inner(range(equal = 2)))]
    inner: &'a [i32],
}

#[test]
fn exact_length_valid() {
    util::check_ok(
        &[Exact {
            field: 2,
            inner: &[2],
        }],
        &(),
    )
}

#[test]
fn exact_length_invalid() {
    util::check_fail!(
        &[
            Exact {
                field: 0,
                inner: &[0]
            },
            Exact {
                field: 1,
                inner: &[1]
            },
            Exact {
                // 'a' * 3
                field: 3,
                inner: &[3]
            },
        ],
        &()
    )
}

#[derive(Debug, garde::Validate)]
struct MinMaxEqual {
    #[garde(range(min = 40, max = 40))]
    min_max: u64,
    #[garde(range(equal = 40))]
    equal: u64,
}

#[test]
fn min_max_equal_length_valid() {
    util::check_ok(
        &[MinMaxEqual {
            min_max: 40,
            equal: 40,
        }],
        &(),
    )
}

#[test]
fn min_max_equal_length_invalid() {
    util::check_fail!(
        &[
            MinMaxEqual {
                min_max: 0,
                equal: 0
            },
            MinMaxEqual {
                min_max: 39,
                equal: 39
            },
            MinMaxEqual {
                min_max: 41,
                equal: 41
            },
        ],
        &()
    )
}
