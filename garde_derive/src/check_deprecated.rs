use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote_spanned, ToTokens};

use crate::model::{self, RawRule};

const BYTE_LENGTH_DEPRECATED: &str = "the `byte_length` attribute is deprecated. Use `length` instead. (See https://github.com/jprochazk/garde/issues/84)";

pub struct DeprecatedWarningSpans {
    input_ident_name: String,
    spans: Vec<Span>,
}

pub fn check(input: &model::Input) -> DeprecatedWarningSpans {
    let model::Input { ident, kind, .. } = input;

    let mut spans = Vec::new();
    match kind {
        model::InputKind::Struct(variant) => {
            spans.append(&mut check_variant(variant));
        }
        model::InputKind::Enum(list) => {
            for (_, variant) in list {
                if let Some(variant) = variant {
                    spans.append(&mut check_variant(variant));
                }
            }
        }
    };

    DeprecatedWarningSpans {
        input_ident_name: ident.to_string(),
        spans,
    }
}

fn check_variant(variant: &model::Variant) -> Vec<Span> {
    let mut spans = Vec::new();

    match variant {
        model::Variant::Struct(map) => {
            for field in map.values() {
                spans.append(&mut check_field(field));
            }
        }
        model::Variant::Tuple(list) => {
            for field in list {
                spans.append(&mut check_field(field));
            }
        }
    };

    spans
}

fn check_field(field: &model::Field) -> Vec<Span> {
    let model::Field {
        rules: raw_rules, ..
    } = field;

    let mut spans = Vec::new();
    for RawRule { span, kind } in raw_rules {
        if let model::RawRuleKind::ByteLength(_) = kind {
            spans.push(*span);
        }
    }

    spans
}

impl ToTokens for DeprecatedWarningSpans {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for (i, &span) in self.spans.iter().enumerate() {
            let name = Ident::new(
                &format!("__{}_byte_length_deprecated_{i}", self.input_ident_name),
                Span::call_site(),
            );

            // Inspired by:
            // https://github.com/ggwpez/proc-macro-warning/blob/970809f551eb78ea003006ef4da0c303ede8501d/proc-macro-warning/src/lib.rs#L260
            quote_spanned! {span=>
                #[allow(dead_code)]
                fn #name() {
                    #[deprecated(note = #BYTE_LENGTH_DEPRECATED)]
                    #[allow(non_upper_case_globals)]
                    const _w: () = ();
                    let _ = _w;
                }
            }
            .to_tokens(tokens);
        }
    }
}
