#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(byte_length(min = 10, max = 100))]
    field: &'a str,
}

fn main() {}
