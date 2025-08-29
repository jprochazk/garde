//! Error message internationalization (i18n).
//!
//! This module provides a trait-based system for customizing validation error messages
//! in different languages or formats. By implementing the [`I18n`] trait, you can provide
//! custom error messages for validation rules supported by `garde`.
//!
//! # Customization
//!
//! By default, `garde` uses english error messages via the [`DefaultI18n`] implementation.
//!
//! To use custom error messages, implement the [`I18n`] trait, and validate inside a call to [`with_i18n`].
//!
//! ```rust,ignore
//! use garde::{Validate, i18n::{I18n, with_i18n}};
//!
//! struct Czech;
//!
//! impl I18n for Czech {
//!     fn length_lower_than(&self, _: usize, min: usize) -> String {
//!         format!("musí obsahovat alespoň {min} znaků")
//!     }
//!
//!     fn email_invalid(&self, error: &str) -> String {
//!         format!("email je neplatný: {error}")
//!     }
//!
//!     // etc.
//! }
//!
//! #[derive(Validate)]
//! struct User {
//!     #[garde(length(min = 3))]
//!     name: String,
//!     #[garde(email)]
//!     email: String,
//! }
//!
//! let user = User {
//!     name: "Jan Novák".to_string(),
//!     email: "invalid-email".to_string(),
//! };
//!
//! let result = with_i18n(Czech, || user.validate());
//! ```
//!

use std::cell::RefCell;

/// Trait for providing custom validation error messages.
///
/// # Implementation
///
/// ```rust,ignore
/// use garde::i18n::I18n;
///
/// struct SpanishI18n;
///
/// impl I18n for SpanishI18n {
///     fn required_not_set(&self) -> String {
///         "no establecido".to_string()
///     }
///
///     fn length_lower_than(&self, _current: usize, min: usize) -> String {
///         format!("la longitud es menor que {min}")
///     }
///
///     // etc.
/// }
/// ```
pub trait I18n {
    /// Rule: `length`
    fn length_lower_than(&self, min: usize) -> String;

    /// Rule: `length`
    fn length_greater_than(&self, max: usize) -> String;

    /// Rule: `range`
    fn range_lower_than(&self, min: &str) -> String;

    /// Rule: `range`
    fn range_greater_than(&self, max: &str) -> String;

    /// Rule: `credit_card`
    fn credit_card_invalid(&self, error: &str) -> String;

    /// Rule: `pattern`
    fn pattern_no_match(&self, pattern: &str) -> String;

    /// Rule: `contains`
    fn contains_missing(&self, pattern: &str) -> String;

    /// Rule: `url`
    fn url_invalid(&self, error: &str) -> String;

    /// Rule: `prefix`
    fn prefix_missing(&self, pattern: &str) -> String;

    /// Rule: `suffix`
    fn suffix_missing(&self, pattern: &str) -> String;

    /// Rule: `phone_number`
    fn phone_number_invalid(&self) -> String;

    /// Rule: `phone_number`
    fn phone_number_invalid_with_error(&self, error: &str) -> String;

    /// Rule: `ip`
    fn ip_invalid(&self, kind: &str) -> String;

    /// Rule: `matches`
    fn matches_field_mismatch(&self, field: &str) -> String;

    /// Rule: `email`
    fn email_invalid(&self, error: &str) -> String;

    /// Rule: `ascii`
    fn ascii_invalid(&self) -> String;

    /// Rule: `alphanumeric`
    fn alphanumeric_invalid(&self) -> String;

    /// Rule: `required`
    fn required_not_set(&self) -> String;
}

macro_rules! i18n {
    ($handler:ident $(, $($args:expr),*)?) => {
        $crate::i18n::I18N.with_borrow(|i18n| {
            let i18n = i18n.as_deref().unwrap_or(&$crate::i18n::DefaultI18n);
            i18n.$handler($($($args),*)?)
        })
    };
}

/// Default implementation of [`I18n`] which provides english error messages.
pub struct DefaultI18n;

impl I18n for DefaultI18n {
    fn length_lower_than(&self, min: usize) -> String {
        format!("length is lower than {min}")
    }

    fn length_greater_than(&self, max: usize) -> String {
        format!("length is greater than {max}")
    }

    fn range_lower_than(&self, min: &str) -> String {
        format!("lower than {min}")
    }

    fn range_greater_than(&self, max: &str) -> String {
        format!("greater than {max}")
    }

    fn credit_card_invalid(&self, error: &str) -> String {
        format!("not a valid credit card number: {error}")
    }

    fn pattern_no_match(&self, pattern: &str) -> String {
        format!("does not match pattern /{pattern}/")
    }

    fn contains_missing(&self, pattern: &str) -> String {
        format!("does not contain \"{pattern}\"")
    }

    fn url_invalid(&self, error: &str) -> String {
        format!("not a valid url: {error}")
    }

    fn prefix_missing(&self, pattern: &str) -> String {
        format!("value does not begin with \"{pattern}\"")
    }

    fn suffix_missing(&self, pattern: &str) -> String {
        format!("does not end with \"{pattern}\"")
    }

    fn phone_number_invalid(&self) -> String {
        "not a valid phone number".to_owned()
    }

    fn phone_number_invalid_with_error(&self, error: &str) -> String {
        format!("not a valid phone number: {error}")
    }

    fn ip_invalid(&self, kind: &str) -> String {
        format!("not a valid {kind} address")
    }

    fn matches_field_mismatch(&self, field: &str) -> String {
        format!("does not match {field} field")
    }

    fn email_invalid(&self, error: &str) -> String {
        format!("not a valid email: {error}")
    }

    fn ascii_invalid(&self) -> String {
        "not ascii".to_owned()
    }

    fn alphanumeric_invalid(&self) -> String {
        "not alphanumeric".to_owned()
    }

    fn required_not_set(&self) -> String {
        "not set".to_owned()
    }
}

thread_local! {
    pub(crate) static I18N: RefCell<Option<Box<dyn I18n>>> = const { RefCell::new(None) };
}

/// Execute a closure with a custom i18n handler.
///
/// This function temporarily sets a custom [`I18n`] implementation for the current thread
/// and executes the provided closure. The handler is automatically restored to the default
/// when the closure completes.
///
/// ## Example
///
/// ```rust,ignore
/// struct CustomI18n;
/// impl garde::I18n for CustomI18n {
///     fn required_not_set(&self) -> String {
///         "This field is required".to_string()
///     }
///
///     // etc.
/// }
///
/// #[derive(garde::Validate)]
/// struct Data {
///     #[garde(required)]
///     field: Option<String>,
/// }
///
/// let data = Data { field: None };
/// let result = garde::with_i18n(CustomI18n, || data.validate());
/// # _ = result;
/// ```
///
pub fn with_i18n<R>(handler: impl I18n + 'static, f: impl FnOnce() -> R) -> R {
    I18N.set(Some(Box::new(handler)));
    let r = f();
    I18N.set(None);
    r
}
