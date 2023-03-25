#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(unknown_rule)]
    field: &'a str,
}

fn main() {}
