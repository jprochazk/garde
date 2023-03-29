#[derive(Debug, garde::Validate)]
struct Inner<'a> {
    #[garde(length(min = 1))]
    field: &'a str,
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(dive, length(min = 1))]
    field: Inner<'a>,
}

fn main() {}
