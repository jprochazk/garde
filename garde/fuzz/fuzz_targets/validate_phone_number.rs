#![no_main]

use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

use garde::Validate;

#[derive(Arbitrary, Validate, Debug)]
struct PhoneNumber {
    #[garde(phone_number)]
    phone_number: String,
}

fuzz_target!(|data: PhoneNumber| {
    let _ = data.validate(&());
});