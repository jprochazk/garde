use super::util;

struct Context {
    needle: String,
}

#[derive(Debug, garde::Validate)]
#[garde(context(Context as ctx))]
#[garde(custom(custom_validate_struct))]
struct Test<'a> {
    #[garde(custom(custom_validate_fn))]
    a: &'a str,
    #[garde(custom(|value: &str, ctx: &Context| {
        if value != ctx.needle {
            return Err(garde::Error::new(format!("`b` is not equal to {}", ctx.needle)));
        }
        Ok(())
    }))]
    b: &'a str,
    #[garde(inner(custom(custom_validate_fn)))]
    inner_a: &'a [&'a str],
    #[garde(inner(custom(|value: &str, ctx: &Context| {
        if value != ctx.needle {
            return Err(garde::Error::new(format!("`b` is not equal to {}", ctx.needle)));
        }
        Ok(())
    })))]
    inner_b: &'a [&'a str],

    #[garde(length(min = ctx.needle.len()))]
    uses_ctx: &'a str,
}

fn custom_validate_struct(test: &Test, _ctx: &Context) -> Result<(), garde::Error> {
    if test.a != test.uses_ctx {
        return Err(garde::Error::new("`a` is not equal to `uses_ctx`"));
    }
    Ok(())
}

fn custom_validate_fn(value: &str, ctx: &Context) -> Result<(), garde::Error> {
    if value != ctx.needle {
        return Err(garde::Error::new(format!("not equal to {}", ctx.needle)));
    }
    Ok(())
}

#[test]
fn custom_valid() {
    let ctx = Context {
        needle: "test".into(),
    };
    util::check_ok(
        &[Test {
            a: "test",
            b: "test",
            inner_a: &["test"],
            inner_b: &["test"],
            uses_ctx: "test",
        }],
        &ctx,
    )
}

#[test]
fn custom_invalid() {
    let ctx = Context {
        needle: "test".into(),
    };
    util::check_fail!(
        &[Test {
            a: "asdf",
            b: "asdf",
            inner_a: &["asdf"],
            inner_b: &["asdf"],
            uses_ctx: "",
        }],
        &ctx
    )
}

#[derive(Debug, garde::Validate)]
#[garde(context(Context))]
#[garde(custom(custom_validate_multi))]
#[garde(custom(|multi: &Multi, ctx: &Context| {
    if multi.inner.iter().any(|&s| s != ctx.needle) {
        return Err(garde::Error::new("`inner` contains a value not equal to `needle`"));
    }
    Ok(())
}))]
struct Multi<'a> {
    #[garde(custom(custom_validate_fn), custom(custom_validate_fn))]
    field: &'a str,

    #[garde(inner(custom(custom_validate_fn), custom(custom_validate_fn)))]
    inner: &'a [&'a str],
}

fn custom_validate_multi(multi: &Multi, ctx: &Context) -> Result<(), garde::Error> {
    if multi.field != ctx.needle {
        return Err(garde::Error::new("`field` is not equal to `needle`"));
    }
    Ok(())
}

#[test]
fn multi_custom_valid() {
    let ctx = Context {
        needle: "test".into(),
    };
    util::check_ok(
        &[Multi {
            field: "test",
            inner: &["test"],
        }],
        &ctx,
    )
}

#[test]
fn multi_custom_invalid() {
    let ctx = Context {
        needle: "test".into(),
    };
    util::check_fail!(
        &[Multi {
            field: "asdf",
            inner: &["asdf"]
        }],
        &ctx
    )
}
