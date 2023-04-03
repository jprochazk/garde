set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

# Run all tests.
test:
  cargo test -p garde
  cargo test -p garde_tests
  cargo test -p axum_garde --all-features

# Run garde unit and doc tests
test-unit:
  cargo test -p garde

# Run tests using `trybuild`. These can be quite slow, as each test involves invoking `rustc`.
test-ui:
  TRYBUILD=overwrite cargo test -p garde_tests ui

# Run rule E2E tests. This also outputs validation error snapshots which may be reviewed using `insta`.
test-rules:
  cargo insta test -p garde_tests --review -- rules

# Run `axum_garde` tests.
test-axum-garde:
  cargo test -p axum_garde --all-features

# Release a new version of the crate.
release: test
  cargo ws publish --force '*'
