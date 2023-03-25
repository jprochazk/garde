#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(alphanumeric)]
    field: &'a str,
}

fn main() {}
