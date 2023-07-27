#[derive(garde::Validate)]
struct Test<'a> {
    #[garde(ip)]
    a: &'a str,
    #[garde(ipv4)]
    b: &'a str,
    #[garde(ipv6)]
    c: &'a str,
    #[garde(inner(ip))]
    inner_a: &'a [&'a str],
    #[garde(inner(ipv4))]
    inner_b: &'a [&'a str],
    #[garde(inner(ipv6))]
    inner_c: &'a [&'a str],
}

fn main() {}
