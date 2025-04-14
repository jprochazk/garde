use garde::Validate;
use url::Url;

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
    Unit,
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
            Enum::Unit,
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
    println!("{:?}", value.validate().unwrap_err());
}

mod length {
    use super::*;

    #[derive(Debug, Validate)]
    struct Struct {
        #[garde(length(min = 30, max = 100))]
        default: Url,
        #[garde(length(simple, min = 30, max = 100))]
        simple: Url,
        #[garde(length(bytes, min = 30, max = 100))]
        bytes: Url,
        #[garde(length(chars, min = 30, max = 100))]
        chars: Url,
        #[garde(length(graphemes, min = 30, max = 100))]
        graphemes: Url,
        #[garde(length(utf16, min = 30, max = 100))]
        utf_16_code_units: Url,
    }

    #[derive(Debug, Validate)]
    struct Tuple(#[garde(length(min = 30, max = 100))] Url);

    #[derive(Debug, Validate)]
    #[allow(clippy::large_enum_variant)]
    enum Enum {
        Unit,
        Struct {
            #[garde(length(min = 30, max = 100))]
            field: Url,
            #[garde(dive)]
            v: Struct,
        },
        Tuple(#[garde(length(min = 30, max = 100))] Url),
    }

    #[test]
    fn url_valid() {
        let url: Url = "http://info.cern.ch/hypertext/WWW/TheProject.html"
            .parse()
            .unwrap();

        util::check_ok(
            &[Struct {
                default: url.clone(),
                simple: url.clone(),
                bytes: url.clone(),
                chars: url.clone(),
                graphemes: url.clone(),
                utf_16_code_units: url.clone(),
            }],
            &(),
        )
    }

    #[test]
    fn url_tuple_valid() {
        util::check_ok(
            &[Tuple(
                "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
                    .parse()
                    .unwrap(),
            )],
            &(),
        )
    }

    #[test]
    fn url_enum_valid() {
        let url: Url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
            .parse()
            .unwrap();
        util::check_ok(
            &[
                Enum::Unit,
                Enum::Struct {
                    field: url.clone(),
                    v: Struct {
                        default: url.clone(),
                        simple: url.clone(),
                        bytes: url.clone(),
                        chars: url.clone(),
                        graphemes: url.clone(),
                        utf_16_code_units: url.clone(),
                    },
                },
                Enum::Tuple(url.clone()),
            ],
            &(),
        )
    }

    #[test]
    fn url_invalid_short() {
        let short_url: Url = "https://www.youtube.com".parse().unwrap();
        util::check_fail!(
            &[Struct {
                default: short_url.clone(),
                simple: short_url.clone(),
                bytes: short_url.clone(),
                chars: short_url.clone(),
                graphemes: short_url.clone(),
                utf_16_code_units: short_url.clone(),
            }],
            &()
        );
    }

    #[test]
    fn url_invalid_long() {
        let long_url: Url = "https://www.youtube.com/watch?v=dQw4w9WgXcQdQw4w9WgXcQdQw4w9WgXcQdQw4w9WgXcQdQw4w9WgXcQdQw4w9WgXcQdQw4w9WgXcQdQw4w9WgXcQdQw4w9WgXcQdQw4w9WgXcQdQw4w9WgXcQ".parse().unwrap();
        util::check_fail!(
            &[Struct {
                default: long_url.clone(),
                simple: long_url.clone(),
                bytes: long_url.clone(),
                chars: long_url.clone(),
                graphemes: long_url.clone(),
                utf_16_code_units: long_url.clone(),
            }],
            &()
        );
    }

    #[test]
    fn url_tuple_invalid() {
        util::check_fail!(&[Tuple("https://youtube.com".parse().unwrap())], &())
    }

    #[test]
    fn url_enum_invalid() {
        let url: Url = "https://www.youtube.com".parse().unwrap();

        util::check_fail!(
            &[
                Enum::Struct {
                    field: url.clone(),
                    v: Struct {
                        default: url.clone(),
                        simple: url.clone(),
                        bytes: url.clone(),
                        chars: url.clone(),
                        graphemes: url.clone(),
                        utf_16_code_units: url.clone(),
                    },
                },
                Enum::Tuple(url.clone()),
            ],
            &(),
        )
    }

    #[test]
    fn url_valid_wrapper() {
        let url: Url = "https://www.youtube.com".parse().unwrap();
        let value = Struct {
            default: url.clone(),
            simple: url.clone(),
            bytes: url.clone(),
            chars: url.clone(),
            graphemes: url.clone(),
            utf_16_code_units: url.clone(),
        };
        println!("{:?}", value.validate().unwrap_err());
    }
}
