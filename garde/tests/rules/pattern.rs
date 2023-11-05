use super::util;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
mod sub {
    use once_cell::sync::Lazy;
    use regex::Regex;

    pub static LAZY_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^abcd|efgh$").unwrap());
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(pattern(r"^abcd|efgh$"))]
    field: &'a str,

    // Lazy-statics aren't really possible with `js-sys` values since they are `!Sync`
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    #[garde(pattern(sub::LAZY_RE))]
    field_path: &'a str,

    #[garde(pattern(create_regex()))]
    field_call: &'a str,

    #[garde(inner(pattern(r"^abcd|efgh$")))]
    inner: &'a [&'a str],
}

#[cfg(not(all(feature = "js-sys", target_arch = "wasm32", target_os = "unknown")))]
fn create_regex() -> ::regex::Regex {
    ::regex::Regex::new(r"^abcd|efgh$").unwrap()
}

#[cfg(all(feature = "js-sys", target_arch = "wasm32", target_os = "unknown"))]
fn create_regex() -> ::js_sys::RegExp {
    ::js_sys::RegExp::new(r"^abcd|efgh$", "u")
}

#[cfg_attr(not(all(target_arch = "wasm32", target_os = "unknown")), test)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    wasm_bindgen_test::wasm_bindgen_test
)]
fn pattern_valid() {
    util::check_ok(
        &[
            Test {
                field: "abcd",
                #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
                field_path: "abcd",
                field_call: "abcd",
                inner: &["abcd"],
            },
            Test {
                field: "efgh",
                #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
                field_path: "efgh",
                field_call: "efgh",
                inner: &["efgh"],
            },
        ],
        &(),
    )
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[test]
fn pattern_invalid() {
    util::check_fail!(
        &[
            Test {
                field: "dcba",
                field_path: "dcba",
                field_call: "dcba",
                inner: &["dcba"]
            },
            Test {
                field: "hgfe",
                field_path: "hgfe",
                field_call: "hgfe",
                inner: &["hgfe"]
            }
        ],
        &()
    )
}
