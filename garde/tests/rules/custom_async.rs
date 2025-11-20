use super::async_util;
use std::time::Duration;
use tokio::time::sleep;

struct Context {
    needle: String,
}

#[derive(Debug, garde::AsyncValidate)]
#[garde(context(Context as ctx))]
struct Test<'a> {
    #[garde(custom(custom_validate_fn))]
    a: &'a str,

    #[garde(custom(async_custom_validate_b))]
    b: &'a str,

    #[garde(length(min = ctx.needle.len()))]
    uses_ctx: &'a str,
}


async fn custom_validate_fn(value: &str, ctx: &Context) -> Result<(), garde::Error> {
    sleep(Duration::from_millis(5)).await;

    if value != ctx.needle {
        return Err(garde::Error::new(format!("not equal to {}", ctx.needle)));
    }
    Ok(())
}

async fn async_custom_validate_b(value: &str, ctx: &Context) -> Result<(), garde::Error> {
    sleep(Duration::from_millis(5)).await;

    if value != ctx.needle {
        return Err(garde::Error::new(format!(
            "`b` is not equal to {}",
            ctx.needle
        )));
    }
    Ok(())
}

#[tokio::test]
async fn custom_valid() {
    let ctx = Context {
        needle: "test".into(),
    };

    async_util::check_ok(
        &[Test {
            a: "test",
            b: "test",
            uses_ctx: "test",
        }],
        &ctx,
    )
    .await;
}

#[tokio::test]
async fn custom_invalid() {
    let ctx = Context {
        needle: "test".into(),
    };

    async_util::async_check_fail!(
        &[Test {
            a: "asdf",
            b: "asdf",
            uses_ctx: "",
        }],
        &ctx
    );
}

//
// Multi tests
//

#[derive(Debug, garde::AsyncValidate)]
#[garde(context(Context))]
struct Multi<'a> {
    #[garde(custom(custom_validate_fn), custom(custom_validate_fn))]
    field: &'a str,

    // #[garde(inner(custom(custom_validate_fn), custom(custom_validate_fn)))]
    // inner: &'a [&'a str],
}

#[tokio::test]
async fn multi_custom_valid() {
    let ctx = Context {
        needle: "test".into(),
    };

    async_util::check_ok(
        &[Multi {
            field: "test",
            // inner: &["test"],
        }],
        &ctx,
    )
    .await;
}

#[tokio::test]
async fn multi_custom_invalid() {
    let ctx = Context {
        needle: "test".into(),
    };

    async_util::async_check_fail!(
        &[Multi {
            field: "asdf",
            // inner: &["asdf"],
        }],
        &ctx
    );
}
