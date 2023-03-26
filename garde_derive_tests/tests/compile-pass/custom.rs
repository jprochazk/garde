#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(custom(custom_validate_fn))]
    a: &'a str,
    #[garde(custom(|_, _| Ok(())))]
    b: &'a str,
}

fn custom_validate_fn(_: &str, _: &()) -> Result<(), garde::Error> {
    unimplemented!()
}

fn main() {}
