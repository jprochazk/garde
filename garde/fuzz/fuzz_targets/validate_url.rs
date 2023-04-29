#![no_main]

use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

use garde::Validate;

#[derive(Arbitrary, Validate, Debug)]
struct Url {
    #[garde(url)]
    url: String,
}

fuzz_target!(|data: Url| {
    let _ = data.validate(&());
});