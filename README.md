# Garde

Rust struct validation library

```rust
#[derive(garde::Validate)]
struct User {
  #[garde(ascii, length(min=3, max=25))]
  username: String,
  #[garde(length(min=15))]
  password: String,
}
```

### 

### Feature flags

- `derive` - the `derive(Validate)` macro
- `url` - validation of URLs via the `url` crate.
- `email` - validation of emails according to the [HTML5 specification](https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address).
- `pattern` - validation using regular expressions via the `regex` crate.
- `credit-card` - validation of credit card numbers via the `card-validate` crate.
- `phone-number` - validation of phone numbers via the `phonenumber` crate.
- `nightly-error-messages` enables usage of `rustc_on_unimplemented` for better error messages. This requires a nightly compiler.

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

This crate is heavily inspired by the [validator](https://github.com/Keats/validator) crate. It is essentially a full rewrite of `validator`. The creation of this crate was prompted by [this comment](https://github.com/Keats/validator/issues/201#issuecomment-1167018511) and a few others talking about a potential rewrite.
