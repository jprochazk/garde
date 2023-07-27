#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(credit_card)]
    field: &'a str,
    #[garde(inner(credit_card))]
    inner: &'a [&'a str],
}

fn main() {}
