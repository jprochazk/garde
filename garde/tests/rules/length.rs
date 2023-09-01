use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 10, max = 100))]
    field: &'a str,
    #[garde(inner(length(min = 10, max = 100)))]
    inner: &'a [&'a str],
}

/*
impl<'a> ::garde::Validate for Test<'a> {
    type Context = ();
    #[allow(clippy::needless_borrow)]
    fn validate_into(
        &self,
        __garde_user_ctx: &Self::Context,
        __garde_path: &::garde::error::Path,
        __garde_report: &mut ::garde::error::Report,
    ) {
        let __garde_user_ctx = &__garde_user_ctx;
        {
            let Self { field, inner } = self;
            {
                {
                    let __garde_path = &__garde_path.join("field");
                    let __garde_binding = &*field;
                    {
                        if let Err(__garde_error) =
                            (::garde::rules::length::apply)(&*__garde_binding, (10usize, 100usize))
                        {
                            __garde_report.append(__garde_path.clone(), __garde_error);
                        }
                    }
                }
                {
                    let __garde_path = &__garde_path.join("inner");
                    let __garde_binding = &*inner;
                    ::garde::rules::inner::apply(
                        &*__garde_binding,
                        __garde_user_ctx,
                        __garde_path,
                        __garde_report,
                        |__garde_binding, __garde_user_ctx, __garde_path, __garde_report| {
                            if let Err(__garde_error) = (::garde::rules::length::apply)(
                                &*__garde_binding,
                                (10usize, 100usize),
                            ) {
                                __garde_report.append(__garde_path.clone(), __garde_error);
                            }
                        },
                    );
                }
            }
        }
    }
}
*/

/* impl<'a> ::garde::Validate for Test<'a> {
    type Context = ();

    #[allow(clippy::needless_borrow)]
    fn validate_into(
        &self,
        __garde_user_ctx: &Self::Context,
        __garde_path: &::garde::error::Path,
        __garde_report: &mut ::garde::error::Report,
    ) {
        let __garde_user_ctx = &__garde_user_ctx;
        let Self { field, inner } = self;
        {
            let __garde_path = &__garde_path.join("field");
            let __garde_binding = &*field;
            {
                if let Err(__garde_error) =
                    (::garde::rules::length::apply)(&*__garde_binding, (10usize, 100usize))
                {
                    __garde_report.append(__garde_path.clone(), __garde_error);
                }
            }
        }
        {
            let __garde_path = &__garde_path.join("inner");
            let __garde_binding = &*inner;
            {
                ::garde::rules::inner::apply(
                    &*__garde_binding,
                    __garde_user_ctx,
                    __garde_path,
                    __garde_report,
                    |__garde_binding, __garde_user_ctx, __garde_path, __garde_report| {
                        if let Err(__garde_error) =
                            (::garde::rules::length::apply)(&*__garde_binding, (10usize, 100usize))
                        {
                            __garde_report.append(__garde_path.clone(), __garde_error);
                        }
                    },
                );
            }
        }
    }
} */

#[test]
fn length_valid() {
    util::check_ok(&[
        Test {
            // 'a' * 10
            field: "aaaaaaaaaa",
            inner: &["aaaaaaaaaa"]
        },
        Test {
            // 'a' * 100
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            inner: &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]
        },
    ], &())
}

#[test]
fn length_invalid() {
    util::check_fail!(&[
        Test {
            // 'a' * 9
            field: "aaaaaaaaa",
            inner: &["aaaaaaaaa"]
        },
        Test {
            // 'a' * 101
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            inner: &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]
        },
    ], &())
}

#[derive(Debug, garde::Validate)]
struct Exact<'a> {
    #[garde(length(min = 2, max = 2))]
    field: &'a str,
    #[garde(inner(length(min = 2, max = 2)))]
    inner: &'a [&'a str],
}

#[test]
fn exact_length_valid() {
    util::check_ok(
        &[Exact {
            // 'a' * 2
            field: "aa",
            inner: &["aa"],
        }],
        &(),
    )
}

#[test]
fn exact_length_invalid() {
    util::check_fail!(
        &[
            Exact {
                field: "",
                inner: &[""]
            },
            Exact {
                // 'a' * 1
                field: "a",
                inner: &["a"]
            },
            Exact {
                // 'a' * 3
                field: "aaa",
                inner: &["aaa"]
            },
        ],
        &()
    )
}
