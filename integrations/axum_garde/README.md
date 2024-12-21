# axum_garde

> [!WARNING]
> ⚠️ **This crate is deprecated in favor of [axum-valid](https://crates.io/crates/axum-valid).** ⚠️

Provide [garde](https://github.com/jprochazk/garde) validation on your
[axum](https://github.com/tokio-rs/axum) application.

# Getting started

The most important element on this library is [`WithValidation`], a composable
[`extractor`] that performs validation over some payload contents.

For most validators to work, the application state should implement [`FromRef`] for `()`:
```rust
#[derive(Clone)]
struct AppState;

impl axum::extract::FromRef<AppState> for () {
    fn from_ref(_: &AppState) {}
}
```

# Features

| Feature               | Description                                                                                    | Default? |
| --------------------- | ---------------------------------------------------------------------------------------------- | -------- |
| `json`                | Enables support for [`axum::extract::Json`]                                                    | ✅       |
| `form`                | Enables support for [`axum::extract::Form`]                                                    | ✅       |
| `query`               | Enables support for [`axum::extract::Query`]                                                   | ✅       |
| `axum-extra`          | Enables support for [`axum_extra::extract::WithRejection`] and [`axum_extra::extract::Cached`] | ❌       |
| `axum-extra-protobuf` | Enables support for [`axum_extra::protobuf::Protobuf`]                                         | ❌       |
| `axum-extra-query`    | Enables support for [`axum_extra::extract::Query`]                                             | ❌       |
| `axum-yaml`           | Enables support for [`axum_yaml::Yaml`]                                                        | ❌       |
| `axum-msgpack`        | Enables support for [`axum_msgpack::MsgPack`] and [`axum_msgpack::MsgPackRaw`]                 | ❌       |

# Useful links

<!-- TBD -->

[`withvalidation`]: crate::WithValidation
[`extractor`]: axum::extract
[`FromRef`]: axum::extract::FromRef
