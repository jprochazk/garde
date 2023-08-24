#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(byte_length(min = 100, max = 10))]
    field: &'a str,
}

fn main() {}
