#![no_main]

use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

use garde::Validate;

#[derive(Arbitrary, Validate, Debug)]
struct StringTypes {
    #[garde(ascii)]
    ascii: String,

    #[garde(alphanumeric)]
    alphanumeric: String,
}

#[derive(Arbitrary, Validate, Debug)]
struct StringPatterns {
    #[garde(contains("abc"))]
    contains: String,

    #[garde(prefix("abc"))]
    prefix: String,

    #[garde(suffix("abc"))]
    suffix: String,
}

#[derive(Arbitrary, Validate, Debug)]
struct InternetProtocols {
    #[garde(ip)]
    ip: String,

    #[garde(ipv4)]
    ipv4: String,

    #[garde(ipv6)]
    ipv6: String,
}

#[derive(Arbitrary, Validate, Debug)]
struct Lengths {
    #[garde(length(min = 1, max = 10))]
    lengthed: String,

    #[garde(byte_length(min = 1, max = 10))]
    byte_lengthed: Vec<u8>,
}

#[derive(Arbitrary, Debug)]
enum General {
    StringTypes(StringTypes),
    StringPatterns(StringPatterns),
    InternetProtocols(InternetProtocols),
    Lengths(Lengths),
}

fuzz_target!(|data: General| {
    let _ = match data {
        General::StringTypes(data) => data.validate(&()),
        General::StringPatterns(data) => data.validate(&()),
        General::InternetProtocols(data) => data.validate(&()),
        General::Lengths(data) => data.validate(&()),
    };
});