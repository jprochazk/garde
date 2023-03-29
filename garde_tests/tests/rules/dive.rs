use std::rc::Rc;
use std::sync::Arc;

use super::util;

#[derive(Clone, Copy, Debug, garde::Validate)]
struct Inner<'a> {
    #[garde(length(min = 1))]
    field: &'a str,
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(dive)]
    field: Inner<'a>,
    #[garde(dive)]
    by_ref: &'a Inner<'a>,
    #[garde(dive)]
    tuples: (Inner<'a>, Inner<'a>),
    #[garde(dive)]
    slice: &'a [Inner<'a>],
    #[garde(dive)]
    array: [Inner<'a>; 1],
    #[garde(dive)]
    array_ref: &'a [Inner<'a>; 1],
    #[garde(dive)]
    boxed: Box<Inner<'a>>,
    #[garde(dive)]
    rc: Rc<Inner<'a>>,
    #[garde(dive)]
    arc: Arc<Inner<'a>>,
}

#[test]
fn email_valid() {
    let inner = Inner { field: "asdf" };
    util::check_ok(
        &[Test {
            field: inner,
            by_ref: &inner,
            tuples: (inner, inner),
            slice: &[inner],
            array: [inner],
            array_ref: &[inner],
            boxed: Box::new(inner),
            rc: Rc::new(inner),
            arc: Arc::new(inner),
        }],
        &(),
    )
}

#[test]
fn email_invalid() {
    let inner = Inner { field: "" };
    util::check_fail!(
        &[Test {
            field: inner,
            by_ref: &inner,
            tuples: (inner, inner),
            slice: &[inner],
            array: [inner],
            array_ref: &[inner],
            boxed: Box::new(inner),
            rc: Rc::new(inner),
            arc: Arc::new(inner),
        }],
        &()
    )
}
