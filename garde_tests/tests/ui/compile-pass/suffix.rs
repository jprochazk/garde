#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(suffix("test"))]
    field: &'a str,
}

fn main() {}
