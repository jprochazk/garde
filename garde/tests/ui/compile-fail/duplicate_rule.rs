#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(ascii, ascii)]
    field: &'a str,
}

fn main() {}
