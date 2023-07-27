#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 10, max = 100))]
    field: &'a str,
    #[garde(length(min = 10, max = 10))]
    field2: &'a str,
    #[garde(inner(length(min = 10, max = 100)))]
    inner: &'a [&'a str],
    #[garde(inner(length(min = 10, max = 10)))]
    inner2: &'a [&'a str],
}

fn main() {}
