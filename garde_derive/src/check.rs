use std::collections::BTreeSet;

use proc_macro2::Span;
use syn::parse_quote;
use syn::spanned::Spanned;

use crate::model;
use crate::model::LengthMode;
use crate::util::{default_ctx_name, MaybeFoldError};

pub fn check(input: model::Input) -> syn::Result<model::Validate> {
    let model::Input {
        ident,
        generics,
        attrs,
        kind,
    } = input;

    let mut error = None;

    if let Err(e) = check_attrs(&attrs) {
        error.maybe_fold(e);
    }

    let context = match get_context(&attrs) {
        Ok(v) => v,
        Err(e) => {
            error.maybe_fold(e);
            (parse_quote!(()), default_ctx_name())
        }
    };

    let transparent = get_transparent_attr(&attrs);

    let options = get_options(&attrs);

    let kind = match kind {
        model::InputKind::Struct(variant) => {
            let variant = match check_variant(variant, &options) {
                Ok(v) => v,
                Err(e) => {
                    error.maybe_fold(e);
                    model::ValidateVariant::empty()
                }
            };
            model::ValidateKind::Struct(variant)
        }
        model::InputKind::Enum(list) => {
            let mut inner_error = None;
            let mut variants = Vec::new();
            for (ident, variant) in list {
                match variant {
                    Some(variant) => match check_variant(variant, &options) {
                        Ok(v) => variants.push((ident, Some(v))),
                        Err(e) => inner_error.maybe_fold(e),
                    },
                    None => variants.push((ident, None)),
                }
            }
            if let Some(inner_error) = inner_error {
                error.maybe_fold(inner_error);
            }
            model::ValidateKind::Enum(variants)
        }
    };

    if let Some(span) = transparent {
        if !is_unary_struct(&kind) {
            error.maybe_fold(syn::Error::new(
                span,
                "transparent structs must have exactly one field",
            ));
        }
    }

    if let Some(error) = error {
        return Err(error);
    }

    Ok(model::Validate {
        ident,
        generics,
        context,
        is_transparent: transparent.is_some(),
        kind,
        options,
    })
}

fn check_attrs(attrs: &[(Span, model::Attr)]) -> syn::Result<()> {
    let mut error = None;

    let mut set = BTreeSet::new();
    for (span, attr) in attrs {
        let d = attr.discriminant();
        if set.contains(&d) {
            error.maybe_fold(syn::Error::new(
                *span,
                format!("duplicate attribute `{}`", attr.name()),
            ));
        }
        set.insert(d);
    }

    match error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

fn get_context(attrs: &[(Span, model::Attr)]) -> syn::Result<(syn::Type, syn::Ident)> {
    #![allow(clippy::single_match)]

    let error = None;
    let mut context = None;

    for (_, attr) in attrs {
        match attr {
            model::Attr::Context(ty, ident) => context = Some((ty, ident)),
            _ => {}
        }
    }

    if let Some(error) = error {
        return Err(error);
    }

    match context {
        Some((ty, id)) => Ok(((**ty).clone(), (*id).clone())),
        None => Ok((parse_quote!(()), default_ctx_name())),
    }
}

fn get_transparent_attr(attrs: &[(Span, model::Attr)]) -> Option<Span> {
    for (span, attr) in attrs {
        if let model::Attr::Transparent = attr {
            return Some(*span);
        }
    }

    None
}

fn is_unary_struct(k: &model::ValidateKind) -> bool {
    match k {
        model::ValidateKind::Struct(model::ValidateVariant::Tuple(fields)) => {
            fields.iter().filter(|field| field.skip.is_none()).count() == 1
        }
        model::ValidateKind::Struct(model::ValidateVariant::Struct(fields)) => {
            fields
                .iter()
                .filter(|(_, field)| field.skip.is_none())
                .count()
                == 1
        }
        _ => false,
    }
}

fn get_options(attrs: &[(Span, model::Attr)]) -> model::Options {
    let mut options = model::Options {
        allow_unvalidated: false,
    };

    for (_, attr) in attrs {
        match attr {
            model::Attr::Context(..) => {}
            model::Attr::AllowUnvalidated => options.allow_unvalidated = true,
            _ => {}
        }
    }

    options
}

fn check_variant(
    variant: model::Variant,
    options: &model::Options,
) -> syn::Result<model::ValidateVariant> {
    let mut error = None;

    let variant = match variant {
        model::Variant::Struct(map) => {
            let mut fields = Vec::new();
            for (ident, field) in map {
                let field = match check_field(field, options) {
                    Ok(v) => v,
                    Err(e) => {
                        error.maybe_fold(e);
                        continue;
                    }
                };
                fields.push((ident, field))
            }
            model::ValidateVariant::Struct(fields)
        }
        model::Variant::Tuple(list) => {
            let mut fields = Vec::new();
            for field in list {
                let field = match check_field(field, options) {
                    Ok(v) => v,
                    Err(e) => {
                        error.maybe_fold(e);
                        continue;
                    }
                };
                fields.push(field);
            }
            model::ValidateVariant::Tuple(fields)
        }
    };

    if let Some(error) = error {
        return Err(error);
    }

    Ok(variant)
}

fn check_field(field: model::Field, options: &model::Options) -> syn::Result<model::ValidateField> {
    let mut error = None;

    let model::Field {
        ty,
        rules: raw_rules,
    } = field;

    let mut field = model::ValidateField {
        ty,
        adapter: None,
        skip: None,
        alias: None,
        message: None,
        code: None,
        dive: None,
        rule_set: model::RuleSet::empty(),
    };

    if raw_rules.is_empty() {
        if options.allow_unvalidated {
            field.skip = Some(Span::call_site());
        } else {
            error.maybe_fold(syn::Error::new(
                field.ty.span(),
                "field has no validation, use `#[garde(skip)]` if this is intentional",
            ));
        }
    }

    field.rule_set = match check_rules(&mut field, raw_rules) {
        Ok(v) => v,
        Err(e) => {
            error.maybe_fold(e);
            model::RuleSet::empty()
        }
    };

    if let Some(span) = field.skip {
        if !field.is_empty() {
            error.maybe_fold(syn::Error::new(
                span,
                "`skip` may not be combined with other rules",
            ))
        }
    }

    if let Some(span) = field.dive {
        if field.rule_set.inner.is_some() {
            error.maybe_fold(syn::Error::new(
                span,
                "`dive` may not be combined with `inner`",
            ))
        }
    }

    if let Some(error) = error {
        return Err(error);
    }

    Ok(field)
}

fn check_rules(
    field: &mut model::ValidateField,
    raw_rules: Vec<model::RawRule>,
) -> syn::Result<model::RuleSet> {
    let mut error = None;
    let mut rule_set = model::RuleSet::empty();
    for raw_rule in raw_rules {
        if let Err(e) = check_rule(field, raw_rule, &mut rule_set, false) {
            error.maybe_fold(e);
        };
    }
    if let Some(error) = error {
        return Err(error);
    }
    Ok(rule_set)
}

fn check_rule(
    field: &mut model::ValidateField,
    raw_rule: model::RawRule,
    rule_set: &mut model::RuleSet,
    is_inner: bool,
) -> syn::Result<()> {
    macro_rules! apply {
        ($name:ident = $value:expr, $span:expr) => {{
            if is_inner {
                return Err(syn::Error::new(
                    $span,
                    concat!("rule `", stringify!($name), "` may not be used in `inner`")
                ));
            }
            match field.$name {
                Some(_) => {
                    return Err(syn::Error::new(
                        $span,
                        concat!("duplicate rule `", stringify!($name), "`"),
                    ))
                }
                None => field.$name = Some($value),
            }
        }};

        ($rule:ident($($inner:expr)?), $span:expr) => {{
            let rule = model::ValidateRule::$rule$(($inner))?;
            let name = rule.name();
            if !rule_set.rules.insert(rule) {
                return Err(syn::Error::new($span, format!("duplicate rule `{name}`")));
            }
        }};
    }

    let span = raw_rule.span;
    use model::RawRuleKind::*;
    match raw_rule.kind {
        Skip => apply!(skip = span, span),
        Adapt(path) => apply!(adapter = path, span),
        Rename(alias) => apply!(alias = alias.value, span),
        Message(message) => apply!(message = message, span),
        Code(code) => apply!(code = code.value, span),
        Dive => apply!(dive = span, span),
        Custom(custom) => rule_set.custom_rules.push(custom),
        Required => apply!(Required(), span),
        Ascii => apply!(Ascii(), span),
        Alphanumeric => apply!(Alphanumeric(), span),
        Email => apply!(Email(), span),
        Url => apply!(Url(), span),
        Ip => apply!(Ip(), span),
        IpV4 => apply!(IpV4(), span),
        IpV6 => apply!(IpV6(), span),
        CreditCard => apply!(CreditCard(), span),
        PhoneNumber => apply!(PhoneNumber(), span),
        Length(v) => {
            let range = check_range_generic(v.range)?;
            match v.mode {
                LengthMode::Simple => apply!(LengthSimple(range), span),
                LengthMode::Bytes => apply!(LengthBytes(range), span),
                LengthMode::Chars => apply!(LengthChars(range), span),
                LengthMode::Graphemes => apply!(LengthGraphemes(range), span),
                LengthMode::Utf16 => apply!(LengthUtf16(range), span),
            }
        }
        Range(v) => apply!(Range(check_range_not_ord(v)?), span),
        Contains(v) => apply!(Contains(v), span),
        Prefix(v) => apply!(Prefix(v), span),
        Suffix(v) => apply!(Suffix(v), span),
        Pattern(v) => apply!(Pattern(check_regex(v)?), span),
        Inner(v) => {
            if rule_set.inner.is_none() {
                rule_set.inner = Some(Box::new(model::RuleSet::empty()));
            }

            let mut error = None;
            for raw_rule in v.contents {
                if let Err(e) = check_rule(field, raw_rule, rule_set.inner.as_mut().unwrap(), true)
                {
                    error.maybe_fold(e);
                }
            }
            if let Some(error) = error {
                return Err(error);
            }
        }
    };

    Ok(())
}

trait CheckRange: Sized {
    fn check_range(self) -> syn::Result<model::ValidateRange<Self>>;
}

fn check_range_generic<L, R>(
    range: model::Range<model::Either<L, R>>,
) -> syn::Result<model::ValidateRange<model::Either<L, R>>>
where
    L: PartialOrd,
{
    macro_rules! map_validate_range {
        ($value:expr, $wrapper:expr) => {{
            match $value {
                model::ValidateRange::GreaterThan(v) => {
                    model::ValidateRange::GreaterThan($wrapper(v))
                }
                model::ValidateRange::LowerThan(v) => model::ValidateRange::LowerThan($wrapper(v)),
                model::ValidateRange::Between(v1, v2) => {
                    model::ValidateRange::Between($wrapper(v1), $wrapper(v2))
                }
            }
        }};
    }

    let range = match (range.span, range.min, range.max) {
        (span, Some(model::Either::Left(min)), Some(model::Either::Left(max))) => {
            map_validate_range!(
                check_range(model::Range {
                    span,
                    min: Some(min),
                    max: Some(max)
                })?,
                model::Either::Left
            )
        }
        (span, Some(model::Either::Left(min)), None) => {
            map_validate_range!(
                check_range(model::Range {
                    span,
                    min: Some(min),
                    max: None,
                })?,
                model::Either::Left
            )
        }
        (span, None, Some(model::Either::Left(max))) => {
            map_validate_range!(
                check_range(model::Range {
                    span,
                    min: None,
                    max: Some(max),
                })?,
                model::Either::Left
            )
        }
        (span, min, max) => check_range_not_ord(model::Range { span, min, max })?,
    };

    Ok(range)
}

fn check_range<T>(range: model::Range<T>) -> syn::Result<model::ValidateRange<T>>
where
    T: PartialOrd,
{
    match (range.min, range.max) {
        (Some(min), Some(max)) if min <= max => Ok(model::ValidateRange::Between(min, max)),
        (Some(_), Some(_)) => Err(syn::Error::new(
            range.span,
            "`min` must be lower than or equal to `max`",
        )),
        (Some(min), None) => Ok(model::ValidateRange::GreaterThan(min)),
        (None, Some(max)) => Ok(model::ValidateRange::LowerThan(max)),
        (None, None) => Err(syn::Error::new(
            range.span,
            "range must have at least one of `min`, `max`",
        )),
    }
}

fn check_range_not_ord<T>(range: model::Range<T>) -> syn::Result<model::ValidateRange<T>> {
    match (range.min, range.max) {
        (Some(min), Some(max)) => Ok(model::ValidateRange::Between(min, max)),
        (Some(min), None) => Ok(model::ValidateRange::GreaterThan(min)),
        (None, Some(max)) => Ok(model::ValidateRange::LowerThan(max)),
        (None, None) => Err(syn::Error::new(
            range.span,
            "range must have at least one of `min`, `max`",
        )),
    }
}

fn check_regex(value: model::Pattern) -> syn::Result<model::ValidatePattern> {
    match value {
        model::Pattern::Lit(lit) => {
            #[cfg(feature = "regex")]
            {
                if let Err(e) = regex::Regex::new(&lit.value) {
                    return Err(syn::Error::new(lit.span, format!("invalid regex: {e}")));
                }
                Ok(model::ValidatePattern::Lit(lit.value))
            }
            #[cfg(not(feature = "regex"))]
            Err(syn::Error::new(
                lit.span,
                "regex feature must be enabled to use literal patterns",
            ))
        }
        model::Pattern::Expr(expr) => Ok(model::ValidatePattern::Expr(expr)),
    }
}
