#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(pattern(r"a|b"))]
    field: &'a str,
}

fn main() {}
