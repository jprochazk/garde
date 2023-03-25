#![allow(dead_code)]

use std::fmt::{Debug, Write};

use ansi_term::{Color, Style};
use garde::Validate;

pub fn check_ok<T: Validate + Debug>(cases: &[T], ctx: &T::Context) {
    let mut some_failed = false;
    for case in cases {
        if let Err(error) = case.validate(ctx) {
            eprintln!(
                "{} input: {case:?}, errors: [{}]",
                Style::new().fg(Color::Red).paint("FAIL"),
                error.to_string().split('\n').collect::<Vec<_>>().join("; ")
            );
            some_failed = true;
        }
    }

    if some_failed {
        panic!("some cases failed, see error output");
    }
}

#[doc(hidden)]
pub fn __check_fail<T: Validate + Debug>(cases: &[T], ctx: &T::Context) -> String {
    let mut some_success = false;
    let mut snapshot = String::new();
    for case in cases {
        if let Err(error) = case.validate(ctx) {
            writeln!(&mut snapshot, "{case:#?}\n{error}\n").unwrap();
        } else {
            eprintln!(
                "{} input: {case:?}",
                Style::new().fg(Color::Red).paint("SUCCESS")
            );
            some_success = true;
        }
    }

    if some_success {
        panic!("some cases did not fail, see error output");
    }

    snapshot
}

#[doc(hidden)]
#[macro_export]
macro_rules! __check_fail {
    ($input:expr, $ctx:expr $(,)?) => {{
        let snapshot = $crate::util::__check_fail($input, $ctx);
        ::insta::assert_snapshot!(snapshot);
    }};
}

pub use crate::__check_fail as check_fail;
