#![no_main]

use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

use garde::Validate;

#[derive(Arbitrary, Validate, Debug)]
struct Email {
    #[garde(email)]
    email: String,
}

fuzz_target!(|data: Email| {
    let _ = data.validate(&());
});