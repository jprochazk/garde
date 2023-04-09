use std::collections::BTreeSet;

use proc_macro2::Span;
use syn::parse_quote;
use syn::spanned::Spanned;

use crate::model;
use crate::util::MaybeFoldError;

pub fn check(input: model::Input) -> syn::Result<model::Validate> {
    let model::Input {
        ident,
        generics,
        attrs,
        kind,
    } = input;

    let mut error = None;

    let context = match get_context(&attrs) {
        Ok(v) => v,
        Err(e) => {
            error.maybe_fold(e);
            parse_quote!(())
        }
    };

    let kind = match kind {
        model::InputKind::Struct(variant) => {
            let variant = match check_variant(variant) {
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
                match check_variant(variant) {
                    Ok(v) => variants.push((ident, v)),
                    Err(e) => inner_error.maybe_fold(e),
                }
            }
            if let Some(inner_error) = inner_error {
                error.maybe_fold(inner_error);
            }
            model::ValidateKind::Enum(variants)
        }
    };

    if let Some(error) = error {
        return Err(error);
    }

    Ok(model::Validate {
        ident,
        generics,
        context,
        kind,
    })
}

fn get_context(attrs: &[(Span, model::Attr)]) -> syn::Result<syn::Type> {
    let error = None;
    let mut context = None;

    for (_, attr) in attrs {
        match attr {
            model::Attr::Context(ty) => context = Some(ty),
        }
    }

    if let Some(error) = error {
        return Err(error);
    }

    match context {
        Some(v) => Ok((**v).clone()),
        None => Ok(parse_quote!(())),
    }
}

fn check_variant(variant: model::Variant) -> syn::Result<model::ValidateVariant> {
    let mut error = None;

    let variant = match variant {
        model::Variant::Struct(map) => {
            let mut fields = Vec::new();
            for (ident, field) in map {
                let field = match check_field(field) {
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
                let field = match check_field(field) {
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

fn check_field(field: model::Field) -> syn::Result<model::ValidateField> {
    let mut error = None;

    let model::Field {
        ty,
        rules: raw_rules,
    } = field;

    let mut field = model::ValidateField {
        ty,
        skip: model::Skip {
            span: Span::call_site(),
            value: false,
        },
        alias: None,
        message: None,
        code: None,
        dive: false,
        rules: BTreeSet::new(),
        custom_rules: Vec::new(),
    };

    for raw_rule in raw_rules {
        let span = raw_rule.span;
        match check_rule(&mut field, raw_rule, 0) {
            Ok(Some(v)) => {
                if field.rules.contains(&v) {
                    error.maybe_fold(syn::Error::new(
                        span,
                        format!("duplicate rule `{}`", v.name()),
                    ));
                    continue;
                }
                field.rules.insert(v);
            }
            Ok(None) => {}
            Err(e) => error.maybe_fold(e),
        }
    }

    if field.is_empty() && !field.skip.value {
        error.maybe_fold(syn::Error::new(
            field.ty.span(),
            "field has no validation, use `#[garde(skip)]` if this is intentional",
        ));
    }

    if field.skip.value && !field.is_empty() {
        error.maybe_fold(syn::Error::new(
            field.skip.span,
            "`skip` may not be combined with other rules",
        ))
    }

    if let Some(error) = error {
        return Err(error);
    }

    Ok(field)
}

fn check_rule(
    field: &mut model::ValidateField,
    raw_rule: model::RawRule,
    depth: usize,
) -> syn::Result<Option<model::ValidateRule>> {
    // TODO: can this be simplified via a macro? there's a ton of duplicated code
    let rule = match raw_rule.kind {
        model::RawRuleKind::Skip => {
            if field.skip.value {
                return Err(syn::Error::new(raw_rule.span, "duplicate rule `skip`"));
            }
            field.skip = model::Skip {
                span: raw_rule.span,
                value: true,
            };
            None
        }
        model::RawRuleKind::Rename(alias) => {
            if field.alias.is_some() {
                return Err(syn::Error::new(raw_rule.span, "duplicate rule `rename`"));
            }
            field.alias = Some(alias.value);
            None
        }
        model::RawRuleKind::Message(message) => {
            if field.message.is_some() {
                return Err(syn::Error::new(raw_rule.span, "duplicate rule `message`"));
            }
            field.message = Some(message);
            None
        }
        model::RawRuleKind::Code(code) => {
            if field.code.is_some() {
                return Err(syn::Error::new(raw_rule.span, "duplicate rule `code`"));
            }
            field.code = Some(code.value);
            None
        }
        model::RawRuleKind::Dive => {
            if field.dive {
                return Err(syn::Error::new(raw_rule.span, "duplicate rule `dive`"));
            }
            field.dive = true;
            None
        }
        model::RawRuleKind::Custom(custom) => {
            field.custom_rules.push(custom.expr());
            None
        }
        model::RawRuleKind::Ascii => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::Ascii,
        }),
        model::RawRuleKind::Alphanumeric => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::Alphanumeric,
        }),
        model::RawRuleKind::Email => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::Email,
        }),
        model::RawRuleKind::Url => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::Url,
        }),
        model::RawRuleKind::Ip => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::Ip,
        }),
        model::RawRuleKind::IpV4 => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::IpV4,
        }),
        model::RawRuleKind::IpV6 => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::IpV6,
        }),
        model::RawRuleKind::CreditCard => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::CreditCard,
        }),
        model::RawRuleKind::PhoneNumber => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::PhoneNumber,
        }),
        model::RawRuleKind::Length(v) => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::Length(check_range(v)?),
        }),
        model::RawRuleKind::ByteLength(v) => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::ByteLength(check_range(v)?),
        }),
        model::RawRuleKind::Range(v) => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::Range(check_range_not_ord(v)?),
        }),
        model::RawRuleKind::Contains(v) => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::Contains(v.value),
        }),
        model::RawRuleKind::Prefix(v) => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::Prefix(v.value),
        }),
        model::RawRuleKind::Suffix(v) => Some(model::ValidateRule {
            depth,
            kind: model::ValidateRuleKind::Suffix(v.value),
        }),
        model::RawRuleKind::Pattern(v) => {
            #[cfg(feature = "regex")]
            {
                if let Err(e) = regex::Regex::new(&v.value) {
                    return Err(syn::Error::new(
                        raw_rule.span,
                        format!("invalid regex: {e}"),
                    ));
                }
            }
            Some(model::ValidateRule {
                depth,
                kind: model::ValidateRuleKind::Pattern(v.value),
            })
        }
        model::RawRuleKind::Inner(v) => {
            return Err(syn::Error::new(
                v.span,
                "`inner` rule is not yet implemented ",
            ));
            // check_rule(field, *v, depth + 1)?
        }
    };

    Ok(rule)
}

trait CheckRange: Sized {
    fn check_range(self) -> syn::Result<model::ValidateRange<Self>>;
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
