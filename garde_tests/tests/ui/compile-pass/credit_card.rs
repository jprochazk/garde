#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(credit_card)]
    field: &'a str,
}

fn main() {}
