use std::path::{Path, PathBuf};

use glob::glob;

const COMPILE_FAIL_TESTS: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/ui/compile-fail/**/*.rs");
const COMPILE_PASS_TESTS: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/ui/compile-pass/**/*.rs");

fn matches(path: &Path, pat: &[String]) -> bool {
    pat.iter()
        .any(|pat| path.as_os_str().to_string_lossy().contains(pat))
}

fn get_filter() -> Vec<String> {
    std::env::var("EXCLUDE_UI_TESTS")
        .unwrap_or_default()
        .split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(String::from)
        .collect::<Vec<_>>()
}

fn get_tests<'a>(path_pattern: &str, filter: &'a [String]) -> impl Iterator<Item = PathBuf> + 'a {
    glob(path_pattern)
        .unwrap()
        .map(Result::unwrap)
        .filter(move |path| !matches(path, filter))
}

#[test]
fn ui() {
    let filter = get_filter();
    let t = trybuild::TestCases::new();
    get_tests(COMPILE_FAIL_TESTS, &filter).for_each(|test| t.compile_fail(test));
    get_tests(COMPILE_PASS_TESTS, &filter).for_each(|test| t.pass(test));
}
