#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 1))]
    v: Option<&'a str>,
}
