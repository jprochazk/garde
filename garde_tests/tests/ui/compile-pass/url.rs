#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(url)]
    field: &'a str,
}

fn main() {}
