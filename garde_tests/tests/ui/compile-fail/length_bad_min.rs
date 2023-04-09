#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 100, max = 10))]
    field: &'a str,
}

fn main() {}
