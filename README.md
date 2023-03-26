# Garde &emsp; [![Documentation]][docs.rs] [![Latest Version]][crates.io]

[docs.rs]: https://docs.rs/garde/latest/garde/
[crates.io]: https://crates.io/crates/garde
[Documentation]: https://img.shields.io/docsrs/garde
[Latest Version]: https://img.shields.io/crates/v/garde.svg

A Rust validation library

```rust
use garde::{Validate, Valid};
use serde::Deserialize;

#[derive(Deserialize, Validate)]
struct User<'a> {
    #[garde(ascii, length(min=3, max=25))]
    username: &'a str,
    #[garde(length(min=15))]
    password: &'a str,
}

let user = serde_json::from_str::<User>(r#"
{
    "username": "lolcode",
    "password": "hunter2"
}
"#).unwrap();

println!("{}", user.validate(&()).unwrap_err());
```

Garde can also validate enums:

```rust
use garde::{Validate, Valid};
use serde::Deserialize;

#[derive(Deserialize, Validate)]
#[serde(rename_all="lowercase")]
enum Data {
    Struct {
        #[garde(range(min=-10, max=10))]
        field: i32,
    },
    Tuple(
        #[garde(rename="important", ascii)]
        String
    ),
}

let data = serde_json::from_str::<Vec<Data>>(r#"
[
    { "struct": { "field": 100 } },
    { "tuple": "test" }
]
"#).unwrap();

for item in &data {
    println!("{}", item.validate(&()).unwrap_err());
}
```

### Available validation rules

| name         | format                                      | validation                                                   | feature flag   |
|--------------|---------------------------------------------|--------------------------------------------------------------|----------------|
| ascii        | `#[garde(ascii)]`                           | only contains ASCII                                          | -              |
| alphanumeric | `#[garde(alphanumeric)]`                    | only letters and digits                                      | -              |
| email        | `#[garde(email)]`                           | an email according to the HTML5 spec[^1]                     | `email`        |
| url          | `#[garde(url)]`                             | a URL                                                        | `url`          |
| ip           | `#[garde(ip)]`                              | an IP address (either IPv4 or IPv6)                          | -              |
| ipv4         | `#[garde(ipv4)]`                            | an IPv4 address                                              | -              |
| ipv6         | `#[garde(ipv6)]`                            | an IPv6 address                                              | -              |
| credit card  | `#[garde(credit_card)]`                     | a credit card number                                         | `credit-card`  |
| phone number | `#[garde(phone_number)]`                    | a phone number                                               | `phone-number` |
| length       | `#[garde(length(min=<usize>, max=<usize>)]` | a dynamically-sized value with size in the range `min..=max` | -              |
| range        | `#[garde(range(min=<expr>, max=<expr>))]`   | a number in the range `min..=max`                            | -              |
| contains     | `#[garde(contains(<string>))]`              | a string-like value containing a substring                   | -              |
| prefix       | `#[garde(prefix(<string>))]`                | a string-like value prefixed by some string                  | -              |
| suffix       | `#[garde(suffix(<string>))]`                | a string-like value suffixed by some string                  | -              |
| pattern      | `#[garde(pattern(<regex>))]`                | a string-like value matching some regular expression         | `pattern`      |
| dive         | `#[garde(dive)]`                            | nested validation, calls `validate` on the value             | -              |
| custom       | `#[garde(custom(<function or closure>))]`   | a custom validator                                           | -              |


Additional notes:
- For `length` and `range`, either `min` or `max` may be omitted, but not both.
- `length` and `range` use an *inclusive* upper bound (`min..=max`).
- `length` uses `.chars().count()` for UTF-8 strings instead of `.len()`.
- For `contains`, `prefix`, and `suffix`, the pattern must be a string literal, because the `Pattern` API [is currently unstable](https://github.com/rust-lang/rust/issues/27721).
- Nested validation using `dive` may not be combined with any other rule.

### Feature flags


| name                     | description                                                                                                                       | extra dependencies                                                                           |
|--------------------------|-----------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------|
| `derive`                 | Enables the usage of the `derive(Validate)` macro                                                                                 | [`garde_derive`](https://crates.io/crates/garde_derive)                                      |
| `url`                    | Validation of URLs via the `url` crate.                                                                                           | [`url`](https://crates.io/crates/url)                                                        |
| `email`                  | Validation of emails according to [HTML5](https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address)                 | [`regex`](https://crates.io/crates/regex), [`once_cell`](https://crates.io/crates/once_cell) |
| `email-idna`             | Support for [Internationalizing Domain Names for Applications](https://url.spec.whatwg.org/#idna) in email addresses              | [`idna`](https://crates.io/crates/idna)                                                      |
| `pattern`                | Validation using regular expressions via the `regex` crate                                                                        | [`regex`](https://crates.io/crates/regex), [`once_cell`](https://crates.io/crates/once_cell) |
| `credit-card`            | Validation of credit card numbers via the `card-validate` crate                                                                   | [`card-validate`](https://crates.io/crates/card-validate)                                    |
| `phone-number`           | Validation of phone numbers via the `phonenumber` crate                                                                           | [`phonenumber`](https://crates.io/crates/phonenumber)                                        |
| `nightly-error-messages` | Enables usage of `rustc_on_unimplemented` for better error messages. This is an unstable feature and requires a nightly compiler. | -                                                                                            |


### Why `Garde`?

Garde means guard in French. I am not French, nor do I speak the language, but `guard` was taken, and this is close enough :).

### License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

### Acknowledgements

This crate is heavily inspired by the [validator](https://github.com/Keats/validator) crate. It is essentially a full rewrite of `validator`.
The creation of this crate was prompted by [this comment](https://github.com/Keats/validator/issues/201#issuecomment-1167018511)
and a few others talking about a potential rewrite.

[^1]: [HTML5 forms - valid email address](https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address)
