#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(alphanumeric)]
    field: &'a str,
    #[garde(inner(alphanumeric))]
    inner: &'a [&'a str],
}

fn main() {}
