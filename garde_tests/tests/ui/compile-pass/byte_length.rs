#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(byte_length(min = 10, max = 100))]
    field: &'a str,
    #[garde(byte_length(min = 10, max = 10))]
    field2: &'a str,
}

fn main() {}
