use criterion::{black_box, criterion_group, criterion_main, Criterion};
use garde::Validate;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(alphanumeric)]
    alphanumeric: Option<&'a str>,
    #[garde(ascii)]
    ascii: Option<&'a str>,
    #[garde(length(min = 1))]
    length_min1_u8_slice: Option<&'a [u8]>,
    #[garde(contains("a"))]
    contains_a: Option<&'a str>,
    #[garde(credit_card)]
    credit_card: Option<&'a str>,
    #[garde(email)]
    email: Option<&'a str>,
    #[garde(ip)]
    ip: Option<&'a str>,
    #[garde(length(min = 1))]
    length_min1: Option<&'a str>,
    #[garde(pattern(r"a|b"))]
    pat_a_or_b: Option<&'a str>,
    #[garde(phone_number)]
    phone_number: Option<&'a str>,
    #[garde(prefix("a"))]
    prefix_a: Option<&'a str>,
    #[garde(range(min = 1))]
    range_min1: Option<i32>,
    #[garde(required)]
    required: Option<&'a str>,
    #[garde(suffix("a"))]
    suffix_a: Option<&'a str>,
    #[garde(url)]
    url: Option<&'a str>,
    #[garde(dive)]
    nested: Option<Box<Test<'a>>>,
}

macro_rules! valid_input {
    () => {
        Test {
            alphanumeric: Some("a"),
            ascii: Some("a"),
            length_min1_u8_slice: Some(&[0]),
            contains_a: Some("a"),
            credit_card: Some("4539571147647251"),
            email: Some("test@mail.com"),
            ip: Some("127.0.0.1"),
            length_min1: Some("a"),
            pat_a_or_b: Some("a"),
            phone_number: Some("+14152370800"),
            prefix_a: Some("a"),
            range_min1: Some(1),
            required: Some("a"),
            suffix_a: Some("a"),
            url: Some("http://test.com"),
            nested: None,
        }
    };
    ($nested:expr) => {
        Test {
            alphanumeric: Some("a"),
            ascii: Some("a"),
            length_min1_u8_slice: Some(&[0]),
            contains_a: Some("a"),
            credit_card: Some("4539571147647251"),
            email: Some("test@mail.com"),
            ip: Some("127.0.0.1"),
            length_min1: Some("a"),
            pat_a_or_b: Some("a"),
            phone_number: Some("+14152370800"),
            prefix_a: Some("a"),
            range_min1: Some(1),
            required: Some("a"),
            suffix_a: Some("a"),
            url: Some("http://test.com"),
            nested: Some(Box::new($nested)),
        }
    };
}

macro_rules! invalid_input {
    () => {
        Test {
            alphanumeric: Some("😂"),
            ascii: Some("😂"),
            length_min1_u8_slice: Some(&[]),
            contains_a: Some("😂"),
            credit_card: Some("😂"),
            email: Some("😂"),
            ip: Some("😂"),
            length_min1: Some(""),
            pat_a_or_b: Some("😂"),
            phone_number: Some("😂"),
            prefix_a: Some(""),
            range_min1: Some(0),
            required: None,
            suffix_a: Some("😂"),
            url: Some("😂"),
            nested: None,
        }
    };
    ($nested:expr) => {
        Test {
            alphanumeric: Some("😂"),
            ascii: Some("😂"),
            length_min1_u8_slice: Some(&[]),
            contains_a: Some("😂"),
            credit_card: Some("😂"),
            email: Some("😂"),
            ip: Some("😂"),
            length_min1: Some(""),
            pat_a_or_b: Some("😂"),
            phone_number: Some("😂"),
            prefix_a: Some(""),
            range_min1: Some(0),
            required: None,
            suffix_a: Some("😂"),
            url: Some("😂"),
            nested: Some(Box::new($nested)),
        }
    };
}

fn validate(c: &mut Criterion) {
    let inputs = vec![
        (
            "valid",
            valid_input!(valid_input!(valid_input!(valid_input!()))),
        ),
        (
            "invalid",
            invalid_input!(invalid_input!(invalid_input!(invalid_input!()))),
        ),
    ];

    for (name, input) in inputs {
        c.bench_function(&format!("validate `{name}`"), |b| {
            b.iter(|| {
                let _ = black_box(input.validate(&()));
            })
        });
    }
}

fn display(c: &mut Criterion) {
    let inputs = vec![
        (
            "valid",
            valid_input!(valid_input!(valid_input!(valid_input!()))).validate(&()),
        ),
        (
            "invalid",
            invalid_input!(invalid_input!(invalid_input!(invalid_input!()))).validate(&()),
        ),
    ];

    for (name, input) in inputs {
        c.bench_function(&format!("display `{name}`"), |b| {
            b.iter(|| {
                let _ = black_box(match &input {
                    Ok(()) => String::new(),
                    Err(e) => e.to_string(),
                });
            })
        });
    }
}

criterion_group!(benches, validate, display);
criterion_main!(benches);
