use super::util;

struct Context {
    needle: String,
}

#[derive(Debug, garde::Validate)]
#[garde(context(Context as ctx))]
struct Test<'a> {
    #[garde(custom(custom_validate_fn))]
    a: &'a str,
    #[garde(custom(|value: &str, ctx: &Context| {
        if value != ctx.needle {
            return Err(garde::Error::new("CUSTOM", format!("`b` is not equal to {}", ctx.needle)));
        }
        Ok(())
    }))]
    b: &'a str,
    #[garde(inner(custom(custom_validate_fn)))]
    inner_a: &'a [&'a str],
    #[garde(inner(custom(|value: &str, ctx: &Context| {
        if value != ctx.needle {
            return Err(garde::Error::new("CUSTOM", format!("`b` is not equal to {}", ctx.needle)));
        }
        Ok(())
    })))]
    inner_b: &'a [&'a str],

    #[garde(length(min = ctx.needle.len()))]
    uses_ctx: &'a str,
}

fn custom_validate_fn(value: &str, ctx: &Context) -> Result<(), garde::Error> {
    if value != ctx.needle {
        return Err(garde::Error::new(
            "CUSTOM",
            format!("not equal to {}", ctx.needle),
        ));
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
struct Multi<'a> {
    #[garde(custom(custom_validate_fn), custom(custom_validate_fn))]
    field: &'a str,

    #[garde(inner(custom(custom_validate_fn), custom(custom_validate_fn)))]
    inner: &'a [&'a str],
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
