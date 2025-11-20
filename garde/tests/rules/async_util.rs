#![allow(dead_code)]

use std::fmt::{Debug, Write};

use garde::{AsyncValidate};
use owo_colors::OwoColorize;

pub async fn check_ok<T: AsyncValidate + Debug>(cases: &[T], ctx: &T::Context) {
    let mut some_failed = false;
    for case in cases {
        if let Err(report) = case.validate_with(ctx).await {
            eprintln!(
                "{} input: {case:?}, errors: [{}]",
                "FAIL".red(),
                report
                    .to_string()
                    .split('\n')
                    .collect::<Vec<_>>()
                    .join("; ")
            );
            some_failed = true;
        }
    }

    if some_failed {
        panic!("some cases failed, see error output");
    }
}

#[doc(hidden)]
pub async fn __check_fail<T: AsyncValidate + Debug>(cases: &[T], ctx: &T::Context) -> String {
    let mut some_success = false;
    let mut snapshot = String::new();
    for case in cases {
        if let Err(report) = case.validate_with(ctx).await {
            writeln!(&mut snapshot, "{case:#?}").unwrap();
            write!(&mut snapshot, "{report}").unwrap();
            writeln!(&mut snapshot).unwrap();
        } else {
            eprintln!("{} input: {case:?}", "SUCCESS".red());
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
macro_rules! __async_check_fail {
    ($input:expr, $ctx:expr $(,)?) => {{
        let snapshot = $crate::rules::async_util::__check_fail($input, $ctx).await;
        ::insta::assert_snapshot!(snapshot);
    }};
}

pub use crate::__async_check_fail as async_check_fail;
