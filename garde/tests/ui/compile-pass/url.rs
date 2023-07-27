#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(url)]
    field: &'a str,
    #[garde(inner(url))]
    inner: &'a [&'a str],
}

fn main() {}
