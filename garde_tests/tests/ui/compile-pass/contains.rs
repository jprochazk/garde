#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(contains("test"))]
    field: &'a str,
}

fn main() {}
