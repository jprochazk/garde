#[derive(garde::Validate)]
struct Test {
    #[garde(range(min = 10, max = 100))]
    field: u64,
}

fn main() {}
