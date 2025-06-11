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
    #[garde(if(cond = ctx.strict_mode, length(min = 8), alphanumeric))]
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
