#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(email)]
    field: &'a str,
    #[garde(inner(email))]
    inner: &'a [&'a str],
}

fn main() {}
