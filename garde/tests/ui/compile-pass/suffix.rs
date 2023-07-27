#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(suffix("test"))]
    field: &'a str,
    #[garde(inner(suffix("test")))]
    inner: &'a [&'a str],
}

fn main() {}
