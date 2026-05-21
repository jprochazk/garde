#[derive(garde::Validate)]
struct SimpleIf {
    #[garde(skip)]
    validate: bool,
    #[garde(if(cond = self.validate, ascii))]
    field: String,
}

#[derive(garde::Validate)]
struct MultipleRulesInIf {
    #[garde(skip)]
    strict: bool,
    #[garde(if(ascii, cond = self.strict, length(min = 3, max = 20)))]
    username: String,
}

#[derive(garde::Validate)]
#[garde(context(Context as ctx))]
struct WithContext {
    #[garde(if(cond = ctx.production, alphanumeric, required))]
    email: Option<String>,
}

struct Context {
    production: bool,
}

#[derive(garde::Validate)]
struct MultipleIfRules {
    #[garde(skip)]
    check_ascii: bool,
    #[garde(skip)]
    check_length: bool,
    #[garde(
        if(cond = self.check_ascii, ascii),
        if(cond = self.check_length, length(min = 5)),
        required
    )]
    value: Option<String>,
}

#[derive(garde::Validate)]
struct ComplexCondition {
    #[garde(skip)]
    is_admin: bool,
    #[garde(skip)]
    is_active: bool,
    #[garde(if(cond = self.is_admin && self.is_active, length(min = 16)))]
    api_key: String,
}

#[derive(garde::Validate)]
#[garde(context(Ctx as ctx))]
struct MixedConditions {
    #[garde(skip)]
    validate_self: bool,
    #[garde(
        if(cond = self.validate_self, ascii),
        if(cond = ctx.strict_mode, length(min = 8))
    )]
    value: String,
}

struct Ctx {
    strict_mode: bool,
}

#[derive(garde::Validate)]
struct ConditionalInner {
    #[garde(skip)]
    validate_items: bool,
    #[garde(if(cond = self.validate_items, inner(ascii, length(min = 2))))]
    items: Vec<String>,
}

#[derive(garde::Validate)]
struct InnerConditional {
    #[garde(skip)]
    check_items: bool,
    #[garde(inner(if(cond = self.check_items, ascii, length(min = 2))))]
    items: Vec<String>,
}

#[derive(garde::Validate)]
struct NestedConditionalInner {
    #[garde(skip)]
    validate_items: bool,
    #[garde(skip)]
    check_ascii: bool,
    #[garde(if(
        cond = self.validate_items,
        inner(if(cond = self.check_ascii, ascii))
    ))]
    items: Vec<String>,
}

struct NestedCtx {
    validate_items: bool,
    check_ascii: bool,
    min_len: usize,
}

#[derive(garde::Validate)]
#[garde(context(NestedCtx as ctx))]
#[garde(custom(validate_contextual_nested_struct))]
struct ContextualNestedConditional<'a> {
    #[garde(if(
        cond = ctx.validate_items,
        inner(if(cond = ctx.check_ascii, ascii), length(min = ctx.min_len))
    ))]
    items: Vec<&'a str>,
}

fn validate_contextual_nested_struct(
    value: &ContextualNestedConditional<'_>,
    ctx: &NestedCtx,
) -> Result<(), garde::Error> {
    if ctx.validate_items && value.items.is_empty() {
        return Err(garde::Error::new("items must not be empty"));
    }
    Ok(())
}

fn main() {}
