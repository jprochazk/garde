use super::util;

#[derive(Debug, garde::Validate)]
struct SimpleConditional {
    #[garde(skip)]
    validate_username: bool,
    #[garde(if(cond = self.validate_username, ascii, length(min = 3)))]
    username: String,
}

#[test]
fn simple_conditional_valid_when_true() {
    util::check_ok(
        &[SimpleConditional {
            validate_username: true,
            username: "test".to_string(),
        }],
        &(),
    )
}

#[test]
fn simple_conditional_valid_when_false() {
    // Should pass validation even with non-ASCII when condition is false
    util::check_ok(
        &[SimpleConditional {
            validate_username: false,
            username: "テスト".to_string(),
        }],
        &(),
    )
}

#[test]
fn simple_conditional_invalid_when_true() {
    util::check_fail!(
        &[SimpleConditional {
            validate_username: true,
            username: "テスト".to_string(),
        }],
        &()
    )
}

#[derive(Debug, garde::Validate)]
#[garde(context(ValidationContext as ctx))]
struct WithContext {
    #[garde(if(length(min = 8), cond = ctx.strict_mode, alphanumeric))]
    password: String,

    #[garde(if(cond = self.email_required, email, required))]
    email: Option<String>,

    #[garde(skip)]
    email_required: bool,
}

struct ValidationContext {
    strict_mode: bool,
}

#[test]
fn context_conditional_valid() {
    let ctx = ValidationContext { strict_mode: true };
    util::check_ok(
        &[WithContext {
            password: "SecurePass123".to_string(),
            email: Some("test@example.com".to_string()),
            email_required: true,
        }],
        &ctx,
    )
}

#[test]
fn context_conditional_invalid() {
    let ctx = ValidationContext { strict_mode: true };
    util::check_fail!(
        &[WithContext {
            password: "weak".to_string(),
            email: None,
            email_required: true,
        }],
        &ctx
    )
}

#[test]
fn context_conditional_valid_when_false() {
    let ctx = ValidationContext { strict_mode: false };
    util::check_ok(
        &[WithContext {
            password: "weak!".to_string(),
            email: None,
            email_required: false,
        }],
        &ctx,
    )
}

#[derive(Debug, garde::Validate)]
struct MultipleConditions {
    #[garde(skip)]
    check_format: bool,
    #[garde(skip)]
    check_length: bool,

    #[garde(
        if(cond = self.check_format, ascii),
        if(cond = self.check_length, length(min = 5, max = 20)),
        required  // Unconditional rule
    )]
    value: Option<String>,
}

#[test]
fn multiple_conditions_all_true() {
    util::check_ok(
        &[MultipleConditions {
            check_format: true,
            check_length: true,
            value: Some("hello".to_string()),
        }],
        &(),
    )
}

#[test]
fn multiple_conditions_mixed() {
    // Format check disabled, length check enabled
    util::check_ok(
        &[MultipleConditions {
            check_format: false,
            check_length: true,
            value: Some("こんにちは".to_string()), // Non-ASCII but length is OK
        }],
        &(),
    )
}

#[test]
fn multiple_conditions_unconditional_fail() {
    // Required rule always applies
    util::check_fail!(
        &[MultipleConditions {
            check_format: false,
            check_length: false,
            value: None,
        }],
        &()
    )
}

#[derive(Debug, garde::Validate)]
struct ComplexCondition {
    #[garde(skip)]
    is_admin: bool,
    #[garde(skip)]
    is_active: bool,

    #[garde(if(cond = self.is_admin && self.is_active, length(min = 16)))]
    api_key: String,
}

#[test]
fn complex_condition_valid() {
    util::check_ok(
        &[ComplexCondition {
            is_admin: true,
            is_active: true,
            api_key: "a".repeat(16),
        }],
        &(),
    )
}

#[test]
fn complex_condition_partial_false() {
    // Only one condition is true, so validation should not apply
    util::check_ok(
        &[ComplexCondition {
            is_admin: true,
            is_active: false,
            api_key: "short".to_string(),
        }],
        &(),
    )
}

#[derive(Debug, garde::Validate)]
struct ConditionalInner {
    #[garde(skip)]
    validate_items: bool,

    #[garde(if(cond = self.validate_items, inner(ascii, length(min = 3))))]
    items: Vec<String>,
}

#[test]
fn conditional_inner_validates_items_when_true() {
    util::check_fail!(
        &[ConditionalInner {
            validate_items: true,
            items: vec!["okay".to_string(), "é".to_string()],
        }],
        &()
    )
}

#[test]
fn conditional_inner_skips_items_when_false() {
    util::check_ok(
        &[ConditionalInner {
            validate_items: false,
            items: vec!["é".to_string()],
        }],
        &(),
    )
}

#[derive(Debug, garde::Validate)]
struct InnerConditional {
    #[garde(skip)]
    check_ascii: bool,
    #[garde(skip)]
    check_length: bool,

    #[garde(inner(
        if(cond = self.check_ascii, ascii),
        if(cond = self.check_length, length(min = 3))
    ))]
    items: Vec<String>,
}

#[test]
fn inner_conditional_validates_items_when_true() {
    util::check_fail!(
        &[InnerConditional {
            check_ascii: true,
            check_length: true,
            items: vec!["é".to_string()],
        }],
        &()
    )
}

#[test]
fn inner_conditional_skips_items_when_false() {
    util::check_ok(
        &[InnerConditional {
            check_ascii: false,
            check_length: false,
            items: vec!["é".to_string()],
        }],
        &(),
    )
}

#[derive(Debug, garde::Validate)]
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

#[test]
fn nested_conditional_inner_requires_both_conditions() {
    util::check_ok(
        &[NestedConditionalInner {
            validate_items: true,
            check_ascii: false,
            items: vec!["é".to_string()],
        }],
        &(),
    );

    util::check_fail!(
        &[NestedConditionalInner {
            validate_items: true,
            check_ascii: true,
            items: vec!["é".to_string()],
        }],
        &()
    )
}

struct NestedContext {
    validate_items: bool,
    check_ascii: bool,
    min_len: usize,
    require_expected_first: bool,
}

#[derive(Debug, garde::Validate)]
#[garde(context(NestedContext as ctx))]
#[garde(custom(validate_contextual_nested_struct))]
struct ContextualNestedConditional<'a> {
    #[garde(if(
        cond = ctx.validate_items,
        inner(if(cond = ctx.check_ascii, ascii), length(min = ctx.min_len))
    ))]
    items: Vec<&'a str>,

    #[garde(skip)]
    expected_first: &'a str,
}

fn validate_contextual_nested_struct(
    value: &ContextualNestedConditional<'_>,
    ctx: &NestedContext,
) -> Result<(), garde::Error> {
    if ctx.require_expected_first && value.items.first() != Some(&value.expected_first) {
        return Err(garde::Error::new(
            "first item does not match expected value",
        ));
    }
    Ok(())
}

#[test]
fn contextual_nested_conditional_uses_context_and_struct_level_rules() {
    let ctx = NestedContext {
        validate_items: true,
        check_ascii: true,
        min_len: 3,
        require_expected_first: true,
    };

    util::check_fail!(
        &[ContextualNestedConditional {
            items: vec!["é"],
            expected_first: "expected",
        }],
        &ctx
    )
}

#[test]
fn contextual_nested_conditional_skips_contextual_field_rules_when_false() {
    let ctx = NestedContext {
        validate_items: false,
        check_ascii: true,
        min_len: 3,
        require_expected_first: false,
    };

    util::check_ok(
        &[ContextualNestedConditional {
            items: vec!["é"],
            expected_first: "expected",
        }],
        &ctx,
    )
}

#[derive(Debug, garde::Validate)]
struct ConditionalRequiredInner {
    #[garde(skip)]
    require_items: bool,

    #[garde(if(cond = self.require_items, inner(required)))]
    conditional_inner: Vec<Option<String>>,

    #[garde(inner(if(cond = self.require_items, required)))]
    inner_conditional: Vec<Option<String>>,
}

#[test]
fn conditional_required_inner_validates_options_when_true() {
    util::check_fail!(
        &[ConditionalRequiredInner {
            require_items: true,
            conditional_inner: vec![None],
            inner_conditional: vec![None],
        }],
        &()
    )
}

#[test]
fn conditional_required_inner_skips_options_when_false() {
    util::check_ok(
        &[ConditionalRequiredInner {
            require_items: false,
            conditional_inner: vec![None],
            inner_conditional: vec![None],
        }],
        &(),
    )
}

struct CustomContext {
    strict: bool,
    needle: String,
}

#[derive(Debug, garde::Validate)]
#[garde(context(CustomContext as ctx))]
struct ConditionalCustomInner<'a> {
    #[garde(inner(if(cond = ctx.strict, custom(matches_needle))))]
    items: Vec<&'a str>,
}

fn matches_needle(value: &str, ctx: &CustomContext) -> Result<(), garde::Error> {
    if value != ctx.needle {
        return Err(garde::Error::new(format!("not equal to {}", ctx.needle)));
    }
    Ok(())
}

#[test]
fn conditional_custom_inner_receives_context() {
    let ctx = CustomContext {
        strict: true,
        needle: "needle".to_string(),
    };

    util::check_fail!(
        &[ConditionalCustomInner {
            items: vec!["other"],
        }],
        &ctx
    )
}

#[derive(Debug, garde::Validate)]
struct ConditionalAdditionalInner {
    #[garde(skip)]
    strict: bool,

    #[garde(inner(ascii), if(cond = self.strict, inner(length(min = 3))))]
    items: Vec<String>,
}

#[test]
fn conditional_additional_inner_rules_are_combined() {
    util::check_fail!(
        &[ConditionalAdditionalInner {
            strict: true,
            items: vec!["é".to_string()],
        }],
        &()
    )
}

#[test]
fn conditional_additional_inner_skips_extra_rules_when_false() {
    util::check_fail!(
        &[ConditionalAdditionalInner {
            strict: false,
            items: vec!["é".to_string()],
        }],
        &()
    )
}

#[derive(Debug, garde::Validate)]
struct DiveChild {
    #[garde(length(min = 3))]
    value: String,
}

#[derive(Debug, garde::Validate)]
struct DiveWithConditionalSiblingRule {
    #[garde(skip)]
    strict: bool,

    #[garde(dive, if(cond = self.strict, custom(always_invalid_child)))]
    child: DiveChild,
}

fn always_invalid_child(_: &DiveChild, _: &()) -> Result<(), garde::Error> {
    Err(garde::Error::new("conditional child rule failed"))
}

#[test]
fn dive_can_be_combined_with_conditional_sibling_rules() {
    util::check_fail!(
        &[DiveWithConditionalSiblingRule {
            strict: true,
            child: DiveChild {
                value: "x".to_string(),
            },
        }],
        &()
    )
}
