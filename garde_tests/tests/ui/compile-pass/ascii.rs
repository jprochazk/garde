#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(ascii)]
    field: &'a str,
}

fn main() {}
