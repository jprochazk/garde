set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

# Run all tests.
test:
  cargo test

# Run tests using `trybuild`. These can be quite slow, as each test involves invoking `rustc`.
test-ui:
  TRYBUILD=overwrite cargo test ui

# Run rule integration tests. This also outputs validation error snapshots which may be reviewed using `insta`.
test-rules:
  cargo insta test --review -- rules

# Release a new version of the crate.
release: test
  cargo ws publish --force 'garde*'
