#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(prefix("test"))]
    field: &'a str,
    #[garde(inner(prefix("test")))]
    inner: &'a [&'a str],
}

fn main() {}
