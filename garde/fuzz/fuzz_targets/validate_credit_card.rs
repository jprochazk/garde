#![no_main]

use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

use garde::Validate;

#[derive(Arbitrary, Validate, Debug)]
struct CreditCard {
    #[garde(credit_card)]
    number: String,
}

fuzz_target!(|data: CreditCard| {
    let _ = data.validate(&());
});