use super::util;

mod test_adapter {
    #![allow(unused_imports)]

    pub use garde::rules::*;

    pub mod length {
        pub use garde::rules::length::*;

        pub mod simple {
            pub fn apply(v: &str, (min, max): (usize, usize)) -> garde::Result {
                if !(min..=max).contains(&v.len()) {
                    Err(garde::Error::new("CUSTOM", "my custom error message"))
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(adapt(test_adapter), length(min = 1))]
    v: &'a str,
}

#[test]
fn alphanumeric_valid() {
    util::check_ok(&[Test { v: "test" }], &())
}

#[test]
fn alphanumeric_invalid() {
    util::check_fail!(&[Test { v: "" }], &())
}
