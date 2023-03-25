//! Validation library
//!
//! ```rust
//! use garde::{Validate, Valid};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, Validate)]
//! struct User<'a> {
//!     #[garde(ascii, length(min=3, max=25))]
//!     username: &'a str,
//!     #[garde(length(min=15))]
//!     password: &'a str,
//! }
//!
//! let user = serde_json::from_str::<User>(r#"
//! {
//!     "username": "lolcode",
//!     "password": "hunter2"
//! }
//! "#).unwrap();
//!
//! println!("{}", user.validate(&()).unwrap_err());
//! ```
//!
//! Garde can also validate enums:
//!
//! ```rust
//! use garde::{Validate, Valid};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, Validate)]
//! #[serde(rename_all="lowercase")]
//! enum Data {
//!     Struct {
//!         #[garde(range(min=-10, max=10))]
//!         field: i32,
//!     },
//!     Tuple(
//!         #[garde(rename="important", ascii)]
//!         String
//!     ),
//! }
//!
//! let data = serde_json::from_str::<Vec<Data>>(r#"
//! [
//!     { "struct": { "field": 100 } },
//!     { "tuple": "test" }
//! ]
//! "#).unwrap();
//!
//! for item in &data {
//!     println!("{:?}", item.validate(&()));
//! }
//! ```
//!
//! ### Available validation rules
//!
//! | name         | format                                      | validation                                                   | feature flag   |
//! |--------------|---------------------------------------------|--------------------------------------------------------------|----------------|
//! | ascii        | `#[garde(ascii)]`                           | only contains ASCII                                          | -              |
//! | alphanumeric | `#[garde(alphanumeric)]`                    | only letters and digits                                      | -              |
//! | email        | `#[garde(email)]`                           | an email according to the HTML5 spec[^1]                     | `email`        |
//! | url          | `#[garde(url)]`                             | a URL                                                        | `url`          |
//! | ip           | `#[garde(ip)]`                              | an IP address (either IPv4 or IPv6)                          | -              |
//! | ipv4         | `#[garde(ipv4)]`                            | an IPv4 address                                              | -              |
//! | ipv6         | `#[garde(ipv6)]`                            | an IPv6 address                                              | -              |
//! | credit card  | `#[garde(credit_card)]`                     | a credit card number                                         | `credit-card`  |
//! | phone number | `#[garde(phone_number)]`                    | a phone number                                               | `phone-number` |
//! | length       | `#[garde(length(min=<usize>, max=<usize>)]` | a dynamically-sized value with size in the range `min..=max` | -              |
//! | range        | `#[garde(range(min=<expr>, max=<expr>))]`   | a number in the range `min..=max`                            | -              |
//! | contains     | `#[garde(contains(<string>))]`              | a string-like value containing a substring                   | -              |
//! | prefix       | `#[garde(prefix(<string>))]`                | a string-like value prefixed by some string                  | -              |
//! | suffix       | `#[garde(suffix(<string>))]`                | a string-like value suffixed by some string                  | -              |
//! | pattern      | `#[garde(pattern(<regex>))]`                | a string-like value matching some regular expression         | `pattern`      |
//! | custom       | `#[garde(custom(<function or closure>))]`   | a custom validator                                           | -              |
//!
//!
//! Additional notes:
//! - For `length` and `range`, either `min` or `max` may be omitted, but not both.
//! - `length` and `range` use an *inclusive* upper bound (`min..=max`).
//! - `length` uses `.chars().count()` for UTF-8 strings instead of `.len()`.
//! - For `contains`, `prefix`, and `suffix`, the pattern must be a string literal, because the `Pattern` API [is currently unstable](https://github.com/rust-lang/rust/issues/27721).
//!

// TODO: test more error cases using `trybuild`
// TODO: if some rule feature is not enabled, it should `compile_error`
// TODO: custom error messages
// TODO: nested validation (`dive` rule)
// TODO: impl `Validate` for various containers (`HashMap`, `Vec`, etc.)

pub mod error;
pub mod rules;
pub mod validate;

pub use error::{Error, Errors};
#[cfg(feature = "derive")]
pub use garde_derive::Validate;
pub use validate::{Unvalidated, Valid, Validate};
