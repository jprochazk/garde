set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

# Run all tests.
test:
  cargo test -p garde
  cargo test -p axum_garde --all-features

# Run garde unit and doc tests
test-unit:
  cargo test -p garde

# Run tests using `trybuild`. These can be quite slow, as each test involves invoking `rustc`.
test-ui:
  TRYBUILD=overwrite cargo test -p garde ui

# Run rule E2E tests. This also outputs validation error snapshots which may be reviewed using `insta`.
test-rules:
  cargo insta test -p garde --review -- rules

# Run `axum_garde` tests.
test-axum-garde:
  cargo test -p axum_garde --all-features

# Update the version of all crates.
version *args:
  cargo ws version {{args}}

# Release a new version of the crate.
release: test
  # publish `garde_derive`
  cargo publish -p garde_derive
  # publish `garde`
  cargo publish -p garde
  # publish `axum_garde`
  cargo publish -p axum_garde
