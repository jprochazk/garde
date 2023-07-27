#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(contains("test"))]
    field: &'a str,
    #[garde(inner(contains("test")))]
    inner: &'a [&'a str],
}

fn main() {}
