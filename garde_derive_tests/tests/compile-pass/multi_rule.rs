#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(prefix("test"), ascii, length(min = 10, max = 100))]
    field: &'a str,
}

fn main() {}
