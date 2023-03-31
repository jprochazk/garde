use garde::Validate;

#[derive(Validate)]
struct Test {
    #[garde(skip)]
    _field: String,
}

fn main() {}
