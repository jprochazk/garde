static NUM: u32 = 666;

#[derive(garde::Validate)]
struct Test {
    #[garde(contains(NUM))]
    field: String,
}

fn main() {}
