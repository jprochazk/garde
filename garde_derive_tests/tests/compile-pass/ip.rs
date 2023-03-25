#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(ip)]
    a: &'a str,
    #[garde(ipv4)]
    b: &'a str,
    #[garde(ipv6)]
    c: &'a str,
}

fn main() {}
