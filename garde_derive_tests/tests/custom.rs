#[path = "./util/mod.rs"]
mod util;

struct Context<'a> {
    needle: &'a str,
}

#[derive(Debug, garde::Validate)]
#[garde(context(Context<'a>))]
struct Test<'a> {
    #[garde(custom(custom_validate_fn))]
    a: &'a str,
    #[garde(custom(|value: &str, ctx: &Context<'a>| {
        if value != ctx.needle {
            return Err(garde::Error::new(format!("`b` is not equal to {}", ctx.needle)));
        }
        Ok(())
    }))]
    b: &'a str,
}

fn custom_validate_fn(value: &str, ctx: &Context<'_>) -> Result<(), garde::Error> {
    if value != ctx.needle {
        return Err(garde::Error::new(format!("not equal to {}", ctx.needle)));
    }
    Ok(())
}

#[test]
fn custom_valid() {
    let ctx = Context { needle: "test" };
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
    let ctx = Context { needle: "test" };
    util::check_fail!(
        &[Test {
            a: "asdf",
            b: "asdf",
        }],
        &ctx
    )
}
