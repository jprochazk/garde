#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(dive, inner(length(min = 1)))]
    field: &'a [&'a str],
}

fn main() {}
