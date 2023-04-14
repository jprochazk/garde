use super::util;
struct Context {
    needle: String,
}

#[derive(Debug, garde::Validate)]
#[garde(context(Context))]
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
        }],
        &ctx
    )
}

#[derive(Debug, garde::Validate)]
#[garde(context(Context))]
struct Multi<'a> {
    #[garde(custom(custom_validate_fn), custom(custom_validate_fn))]
    field: &'a str,
}

#[test]
fn multi_custom_valid() {
    let ctx = Context {
        needle: "test".into(),
    };
    util::check_ok(&[Multi { field: "test" }], &ctx)
}

#[test]
fn multi_custom_invalid() {
    let ctx = Context {
        needle: "test".into(),
    };
    util::check_fail!(&[Multi { field: "asdf" }], &ctx)
}
