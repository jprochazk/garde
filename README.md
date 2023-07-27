# Garde &emsp; [![Documentation]][docs.rs] [![Latest Version]][crates.io]

[docs.rs]: https://docs.rs/garde/latest/garde/
[crates.io]: https://crates.io/crates/garde
[Documentation]: https://img.shields.io/docsrs/garde
[Latest Version]: https://img.shields.io/crates/v/garde.svg

A Rust validation library

- [Basic usage example](#basic-usage-example)
- [Validation rules](#available-validation-rules)
- [Inner type validation](#inner-type-validation)
- [Handling Option](#handling-option)
- [Custom validation](#custom-validation)
- [Implementing rules](#implementing-rules)
- [Implementing `Validate`](#implementing-validate)
- [Integration with web frameworks](#integration-with-web-frameworks)
- [Feature flags](#feature-flags)
- [Why `garde`?](#why-garde)

### Basic usage example

To get started, use the `Validate` derive macro and add some validation rules to your type.
This generates an implementation of the `Validate` trait for you.
To use it, call the `validate` method on an instance of the type.

Here's what that looks like in full:

```rust
use garde::{Validate, Valid};

#[derive(Validate)]
struct User<'a> {
    #[garde(ascii, length(min=3, max=25))]
    username: &'a str,
    #[garde(length(min=15))]
    password: &'a str,
}

let user = User {
    username: "test",
    password: "not_a_very_good_password",
};

if let Err(e) = user.validate(&()) {
    println!("invalid user: {e}");
}
```

Garde can also validate enums:

```rust
use garde::{Validate, Valid};

#[derive(Validate)]
enum Data {
    Struct {
        #[garde(range(min=-10, max=10))]
        field: i32,
    },
    Tuple(
        #[garde(ascii)]
        String
    ),
}

let data = Data::Struct { field: 100 };
if let Err(e) = data.validate(&()) {
    println!("invalid data: {e}");
}
```

### Available validation rules

| name         | format                                           | validation                                           | feature flag   |
| ------------ | ------------------------------------------------ | ---------------------------------------------------- | -------------- |
| required     | `#[garde(required)]`                             | is value set                                         | -              |
| ascii        | `#[garde(ascii)]`                                | only contains ASCII                                  | -              |
| alphanumeric | `#[garde(alphanumeric)]`                         | only letters and digits                              | -              |
| email        | `#[garde(email)]`                                | an email according to the HTML5 spec[^1]             | `email`        |
| url          | `#[garde(url)]`                                  | a URL                                                | `url`          |
| ip           | `#[garde(ip)]`                                   | an IP address (either IPv4 or IPv6)                  | -              |
| ipv4         | `#[garde(ipv4)]`                                 | an IPv4 address                                      | -              |
| ipv6         | `#[garde(ipv6)]`                                 | an IPv6 address                                      | -              |
| credit card  | `#[garde(credit_card)]`                          | a credit card number                                 | `credit-card`  |
| phone number | `#[garde(phone_number)]`                         | a phone number                                       | `phone-number` |
| length       | `#[garde(length(min=<usize>, max=<usize>)]`      | a container with length in `min..=max`               | -              |
| byte_length  | `#[garde(byte_length(min=<usize>, max=<usize>)]` | a byte sequence with length in `min..=max`           | -              |
| range        | `#[garde(range(min=<expr>, max=<expr>))]`        | a number in the range `min..=max`                    | -              |
| contains     | `#[garde(contains(<string>))]`                   | a string-like value containing a substring           | -              |
| prefix       | `#[garde(prefix(<string>))]`                     | a string-like value prefixed by some string          | -              |
| suffix       | `#[garde(suffix(<string>))]`                     | a string-like value suffixed by some string          | -              |
| pattern      | `#[garde(pattern(<regex>))]`                     | a string-like value matching some regular expression | `pattern`      |
| dive         | `#[garde(dive)]`                                 | nested validation, calls `validate` on the value     | -              |
| skip         | `#[garde(skip)]`                                 | skip validation                                      | -              |
| custom       | `#[garde(custom(<function or closure>))]`        | a custom validator                                   | -              |

Additional notes:
- `required` is only available for `Option` fields.
- For `length` and `range`, either `min` or `max` may be omitted, but not both.
- `length` and `range` use an *inclusive* upper bound (`min..=max`).
- `length` uses `.chars().count()` for UTF-8 strings instead of `.len()`.
- For `contains`, `prefix`, and `suffix`, the pattern must be a string literal, because the `Pattern` API [is currently unstable](https://github.com/rust-lang/rust/issues/27721).

If most of the fields on your struct are annotated with `#[garde(skip)]`, you may use `#[garde(allow_unvalidated)]` instead:

```rust
#[derive(garde::Validate)]
struct Foo<'a> {
    #[garde(length(min = 1))]
    a: &'a str,

    #[garde(skip)]
    b: &'a str, // this field will not be validated
}

#[derive(garde::Validate)]
#[garde(allow_unvalidated)]
struct Bar<'a> {
    #[garde(length(min = 1))]
    a: &'a str,

    b: &'a str, // this field will not be validated
                // note the lack of `#[garde(skip)]`
}
```

### Inner type validation

If you need to validate the "inner" type of a container, such as the `String` in `Vec<String>`, then use the `inner` modifier:

```rust
#[derive(garde::Validate)]
struct Test {
    #[garde(
        length(min = 1),
        inner(ascii, length(min = 1)), // wrap the rule in `inner`
    )]
    items: Vec<String>,
}
```

The above type would fail validation if:
- the `Vec` is empty
- any of the inner `String` elements is empty
- any of the inner `String` elements contains non-ASCII characters

### Handling Option

Every rule works on `Option<T>` fields. The field will only be validated if it is `Some`. If you additionally want to validate that the `Option<T>` field is `Some`, use the `required` rule:

```rust
#[derive(garde::Validate)]
struct Test {
    #[garde(required, ascii, length(min = 1))]
    value: Option<String>,
}
```

The above type would fail validation if:
- `value` is `None`
- the inner `value` is empty
- the inner `value` contains non-ASCII characters

### Custom validation

Validation may be customized via the `custom` rule, and the `context` attribute.

The context may be any type without generic parameters. By default, the context is `()`.

```rust
#[derive(garde::Validate)]
#[garde(context(PasswordContext))]
struct User {
    #[garde(custom(is_strong_password))]
    password: String,
}

struct PasswordContext {
    min_entropy: f32,
    entropy: cracken::password_entropy::EntropyEstimator,
}

fn is_strong_password(value: &str, context: &PasswordContext) -> garde::Result {
    let bits = context.entropy.estimate_password_entropy(value.as_bytes())
        .map(|e| e.mask_entropy)
        .unwrap_or(0.0);
    if bits < context.min_entropy {
        return Err(garde::Error::new("password is not strong enough"));
    }
    Ok(())
}

let ctx = PasswordContext { /* ... */ };
let user = User { /* ... */ };
user.validate(&ctx)?;
```

The validator function may accept the value as a reference to any type which it derefs to.
In the above example, it is possible to use `&str`, because `password` is a `String`, and `String` derefs to `&str`.

### Implementing rules

Say you want to implement length checking for a custom string-like type.
To do this, you would implement the `garde::rules::length::HasLength` trait for it.

```rust
#[repr(transparent)]
pub struct MyString(pub String);

impl garde::rules::length::HasLength for MyString {
    fn length(&self) -> usize {
        self.0.chars().count()
    }
}
#[derive(garde::Validate)]
struct Foo {
    // Now the `length` check may be used with `MyString`
    #[garde(length(min = 1, max = 1000))]
    field: MyString,
}
```

Each rule comes with its own trait that may be implemented by custom types in your code.
They are all available under `garde::rules`.

### Implementing `Validate`

In case you have a container type for which you'd like to support nested validation (using the `#[garde(dive)]` rule),
you may implement `Validate` for it:

```rust
#[repr(transparent)]
struct MyVec<T>(Vec<T>);

impl<T: garde::Validate> garde::Validate for MyVec<T> {
    type Context = T::Context;

    fn validate(&self, ctx: &Self::Context) -> Result<(), garde::Errors> {
        garde::Errors::list(|errors| {
            for item in self.0.iter() {
                errors.push(item.validate(ctx));
            }
        })
        .finish()
    }
}

#[derive(garde::Validate)]
struct Foo {
  #[garde(dive)]
  field: MyVec<Bar>,
}

#[derive(garde::Validate)]
struct Bar {
  #[garde(range(min = 1, max = 10))]
  value: u32,
}
```

To make implementing the trait easier, the `Errors` type supports a nesting builders.
- For list-like or tuple-like data structures, use `Errors::list`, and its `.push` method to attach nested `Errors`.
- For map-like data structures, use `Errors::fields`, and its `.insert` method to attach nested `Errors`.
- For a "flat" error list, use `Errors::simple`, and its `.push` method to attach individual errors.

The `ListErrorBuilder::push` and `ListErrorBuilder::insert` methods will ignore any errors which are empty (via `Errors::is_empty`).

### Integration with web frameworks

- [`axum`](https://crates.io/crates/axum): https://crates.io/crates/axum_garde

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


### Why `garde`?

Garde means guard in French. I am not French, nor do I speak the language, but `guard` was taken, and this is close enough :).

### Development

Contributing to `garde` only requires a somewhat recent version of [`Rust`](https://www.rust-lang.org/learn/get-started).

This repository also makes use of the following tools, but they are optional:
- [`insta`](https://insta.rs/) for snapshot testing ([tests/rules](./garde_derive_tests/tests/rules/)).
- [`just`](https://just.systems/) for running recipes defined in the [`justfile`](./justfile).
  Run `just -l` to see what recipes are available.

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
