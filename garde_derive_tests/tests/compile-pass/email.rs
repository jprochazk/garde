#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(email)]
    field: &'a str,
}

fn main() {}
