struct Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(garde::Validate)]
struct Test {}

fn main() {}
