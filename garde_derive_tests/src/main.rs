use garde::Validate;

#[derive(Validate)]
struct Test {
    #[garde(url)]
    field: String,
}

fn main() {}
