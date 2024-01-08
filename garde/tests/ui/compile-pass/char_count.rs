#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(char_count(min = 10, max = 100))]
    field: &'a str,
    #[garde(char_count(min = 10, max = 10))]
    field2: &'a str,
    #[garde(inner(char_count(min = 10, max = 10)))]
    inner: &'a [&'a str],
}

fn main() {}
