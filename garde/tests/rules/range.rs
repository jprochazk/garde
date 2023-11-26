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
