use garde::Validate;

use super::util;

#[derive(Debug, Validate)]
struct Struct<'a> {
    #[garde(url)]
    field: &'a str,
    #[garde(inner(url))]
    inner: &'a [&'a str],
}

#[derive(Debug, Validate)]
struct Tuple<'a>(#[garde(url)] &'a str);

#[derive(Debug, Validate)]
enum Enum<'a> {
    Struct {
        #[garde(url)]
        field: &'a str,
        #[garde(dive)]
        v: Struct<'a>,
    },
    Tuple(#[garde(url)] &'a str),
}

#[test]
fn url_valid() {
    util::check_ok(
        &[
            Struct {
                field: "http://info.cern.ch/hypertext/WWW/TheProject.html",
                inner: &["http://info.cern.ch/hypertext/WWW/TheProject.html"],
            },
            Struct {
                field: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
                inner: &["https://www.youtube.com/watch?v=dQw4w9WgXcQ"],
            },
        ],
        &(),
    )
}

#[test]
fn url_tuple_valid() {
    util::check_ok(&[Tuple("https://www.youtube.com/watch?v=dQw4w9WgXcQ")], &())
}

#[test]
fn url_enum_valid() {
    util::check_ok(
        &[
            Enum::Struct {
                field: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
                v: Struct {
                    field: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
                    inner: &["https://www.youtube.com/watch?v=dQw4w9WgXcQ"],
                },
            },
            Enum::Tuple("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        ],
        &(),
    )
}

#[test]
fn url_invalid() {
    util::check_fail!(
        &[Struct {
            field: "asdf",
            inner: &["asdf"]
        }],
        &()
    )
}

#[test]
fn url_tuple_invalid() {
    util::check_fail!(
        &[Tuple("htt ps://www.youtube.com/watch?v=dQw4w9WgXcQ")],
        &()
    )
}

#[test]
fn url_enum_invalid() {
    util::check_fail!(
        &[
            Enum::Struct {
                field: "htt ps://www.youtube.com/watch?v=dQw4w9WgXcQ",
                v: Struct {
                    field: "htt ps://www.youtube.com/watch?v=dQw4w9WgXcQ",
                    inner: &["htt ps://www.youtube.com/watch?v=dQw4w9WgXcQ"],
                },
            },
            Enum::Tuple("htt ps://www.youtube.com/watch?v=dQw4w9WgXcQ"),
        ],
        &(),
    )
}

#[test]
fn url_valid_wrapper() {
    let value = Struct {
        field: "htt ps://www.youtube.com/watch?v=dQw4w9WgXcQ",
        inner: &["htt ps://www.youtube.com/watch?v=dQw4w9WgXcQ"],
    };
    println!("{:?}", value.validate(&()).unwrap_err());
}
