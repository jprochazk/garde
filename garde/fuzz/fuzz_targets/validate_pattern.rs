#![no_main]

use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

use garde::Validate;

#[derive(Arbitrary, Validate, Debug)]
struct Pattern {
    #[garde(pattern(r"hello"))]
    regex_1: String,

    #[garde(pattern(r"[b-chm-pP]at|ot"))]
    regex_2: String,

    #[garde(pattern(r"[^i*&2@]"))]
    regex_3: String,
}

fuzz_target!(|data: Pattern| {
    let _ = data.validate(&());
});