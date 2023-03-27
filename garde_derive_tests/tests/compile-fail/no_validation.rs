#[derive(garde::Validate)]
struct Test<'a> {
    field: &'a str,
}

fn main() {}
