use super::util;

#[derive(Clone, Copy, Debug, garde::Validate)]
#[garde(context((usize, usize) as ctx))]
struct Inner<'a> {
    #[garde(length(min = ctx.0, max = ctx.1))]
    field: &'a str,
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(skip)]
    min: usize,
    #[garde(skip)]
    max: usize,
    #[garde(dive((self.min, self.max)))]
    inner: Inner<'a>,
}

#[derive(Debug, garde::Validate)]
#[garde(context((usize, usize)))]
struct Test2<'a> {
    #[garde(dive)]
    inner: Inner<'a>,
}

#[test]
fn valid() {
    let inner = Inner { field: "asdf" };
    util::check_ok(&[Test2 { inner }], &(1, 5));
    util::check_ok(
        &[Test {
            min: 1,
            max: 5,
            inner,
        }],
        &(),
    );
}

#[test]
fn invalid() {
    let inner = Inner { field: "asdfgh" };
    util::check_fail!(&[Test2 { inner }], &(1, 5));
    util::check_fail!(
        &[Test {
            min: 1,
            max: 5,
            inner,
        }],
        &(),
    );
}
