#[derive(garde::Validate)]
struct Struct {
    field: u64,
}

#[derive(garde::Validate)]
struct Tuple(u64);

#[derive(garde::Validate)]
enum Enum {
    Struct { field: u64 },
    Tuple(u64),
}

fn main() {}
