#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(phone_number)]
    field: &'a str,
}

fn main() {}
