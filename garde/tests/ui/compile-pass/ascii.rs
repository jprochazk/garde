#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(ascii)]
    field: &'a str,
    #[garde(inner(ascii))]
    inner: &'a [&'a str],
}

fn main() {}
