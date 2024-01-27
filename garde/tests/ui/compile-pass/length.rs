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

    #[garde(length(simple, min = 1, max = 1))]
    simple: &'a str,
    #[garde(length(bytes, min = 1, max = 1))]
    bytes: &'a str,
    #[garde(length(chars, min = 1, max = 1))]
    chars: &'a str,
    #[garde(length(graphemes, min = 1, max = 1))]
    graphemes: &'a str,
    #[garde(length(utf16, min = 1, max = 1))]
    utf16: &'a str,
}

fn main() {}
