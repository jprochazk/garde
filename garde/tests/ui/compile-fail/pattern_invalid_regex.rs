#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(pattern("("))]
    field: &'a str,
}

fn main() {}
