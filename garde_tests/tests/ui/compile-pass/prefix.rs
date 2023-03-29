#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(prefix("test"))]
    field: &'a str,
}

fn main() {}
