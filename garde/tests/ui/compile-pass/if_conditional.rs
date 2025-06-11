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
    #[garde(if(cond = self.strict, ascii, length(min = 3, max = 20)))]
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

fn main() {}