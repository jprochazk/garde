use std::cell::RefCell;
use std::marker::PhantomData;
use std::str::FromStr as _;

use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

use crate::{model, SyncMarker, ValidationMode};

pub fn emit<M: ValidationMode>(input: model::Validate<M>) -> TokenStream2 {
    input.to_token_stream()
}

impl<M: ValidationMode> ToTokens for model::Validate<M> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.ident;
        let (context_ty, context_ident) = &self.context;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let trait_path = M::validate_path();
        let async_token = M::async_token();
        let parent_fn_path = M::parent_fn_path();

        let ty = Type::<M> {
            is_transparent: self.is_transparent,
            kind: &self.kind,
            _mode: PhantomData,
        };

        quote! {
            impl #impl_generics #trait_path for #ident #ty_generics #where_clause {
                type Context = #context_ty ;
                #[allow(clippy::needless_borrow)]
                #async_token fn validate_into(
                    &self,
                    #context_ident: &Self::Context,
                    mut __garde_path: &mut #parent_fn_path,
                    __garde_report: &mut ::garde::error::Report,
                ) {
                    let __garde_user_ctx = &#context_ident;
                    #ty
                }
            }
        }
        .to_tokens(tokens)
    }
}

struct Type<'a, M: ValidationMode> {
    is_transparent: bool,
    kind: &'a model::ValidateKind,
    _mode: PhantomData<M>,
}

impl<M: ValidationMode> ToTokens for Type<'_, M> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let is_transparent = self.is_transparent;
        match &self.kind {
            model::ValidateKind::Struct(variant) => {
                let bindings = Bindings(variant);
                let validation = Variant::<M> {
                    is_transparent,
                    variant,
                    _mode: PhantomData,
                };
                quote! {{
                    let Self #bindings = self;
                    #validation
                }}
            }
            model::ValidateKind::Enum(variants) => {
                let variants = variants.iter().map(|(name, variant)| {
                    if let Some(variant) = variant {
                        let bindings = Bindings(variant);
                        let validation = Variant::<M> {
                            is_transparent,
                            variant,
                            _mode: PhantomData,
                        };
                        quote!(Self::#name #bindings => #validation)
                    } else {
                        quote!(Self::#name => {})
                    }
                });
                quote! {{
                    match self {
                        #(#variants,)*
                    }
                }}
            }
        }
        .to_tokens(tokens)
    }
}

struct Variant<'a, M: ValidationMode> {
    is_transparent: bool,
    variant: &'a model::ValidateVariant,
    _mode: PhantomData<M>,
}

impl<M: ValidationMode> ToTokens for Variant<'_, M> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let is_transparent = self.is_transparent;
        match &self.variant {
            model::ValidateVariant::Struct(fields) => {
                let fields = Struct::<M> {
                    is_transparent,
                    fields,
                    _mode: PhantomData,
                };
                quote! {{#fields}}
            }
            model::ValidateVariant::Tuple(fields) => {
                let fields = Tuple::<M> {
                    is_transparent,
                    fields,
                    _mode: PhantomData,
                };
                quote! {{#fields}}
            }
        }
        .to_tokens(tokens)
    }
}

struct Struct<'a, M = SyncMarker> {
    is_transparent: bool,
    fields: &'a [(Ident, model::ValidateField)],
    _mode: PhantomData<M>,
}

impl<M: ValidationMode> ToTokens for Struct<'_, M> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        Fields::<_, _, M> {
            0: RefCell::new(Some(
                self.fields
                    .iter()
                    .map(|(key, field)| (Binding::Ident(key), field, key.to_string())),
            )),
            1: |key, value| match self.is_transparent {
                true => quote! {{
                    let point_6 =();
                    #value
                }},
                false => quote! {{
                    let point_3 = ();
                    let mut __garde_path = ::garde::util::nested_path!(__garde_path, #key);
                    #value
                }},
            },
            2: PhantomData,
        }
        .to_tokens(tokens)
    }
}

struct Tuple<'a, M: ValidationMode> {
    is_transparent: bool,
    fields: &'a [model::ValidateField],
    _mode: PhantomData<M>,
}

impl<M: ValidationMode> ToTokens for Tuple<'_, M> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let is_transparent = self.is_transparent;
        Fields::<_, _, M> {
            0: RefCell::new(Some(
                self.fields
                    .iter()
                    .enumerate()
                    .map(|(index, field)| (Binding::Index(index), field, index)),
            )),
            1: move |index, value| match is_transparent {
                true => quote! {{
                    //point 5
                    #value
                }},
                false => quote! {{
                    //point 4
                    let mut __garde_path = ::garde::util::nested_path!(__garde_path, #index);
                    #value
                }},
            },
            2: PhantomData,
        }
        .to_tokens(tokens)
    }
}

struct Inner<'a, M: ValidationMode> {
    rules_mod: &'a TokenStream2,
    rule_set: &'a model::RuleSet,
    _mode: PhantomData<M>,
}

impl<M: ValidationMode> ToTokens for Inner<'_, M> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Inner {
            rules_mod,
            rule_set,
            _mode,
        } = self;

        let outer = match rule_set.has_top_level_rules() {
            true => {
                let rules = Rules::<M> {
                    rules_mod,
                    rule_set,
                    _mode: PhantomData,
                };
                Some(quote! {#rules})
            }
            false => None,
        };
        let inner = rule_set.inner.as_deref().map(|rule_set| Inner::<M> {
            rules_mod,
            rule_set,
            _mode: PhantomData,
        });

        let value = match (outer, inner) {
            (Some(outer), Some(inner)) => quote! {
            //point 9
                #outer
                #inner
            },
            (None, Some(inner)) => quote! {
            //point 8
                #inner
            },
            (Some(outer), None) => outer,
            (None, None) => return,
        };

        let async_token = M::async_token();
        let async_apply_iden = M::apply_fn_iden();
        let await_token = M::await_token();

        //WARN: I am not entirely sure if I can safely us async_apply here because of the #rules_mod
        // I should check properly later
        debug_assert_eq!(rules_mod.to_string(), quote!(::garde::rules).to_string());

        //FIX: inner async is not implemented, so panic
        todo!("Inner Async is not supported currently :(");

        quote! {
            #rules_mod::inner::#async_apply_iden(
                &*__garde_binding,
                #async_token |__garde_binding, __garde_inner_key| {
                    let mut __garde_path = ::garde::util::nested_path!(__garde_path, __garde_inner_key);
                    #value
                }
            )#await_token;
        }
        .to_tokens(tokens)
    }
}

struct Rules<'a, M: ValidationMode> {
    rules_mod: &'a TokenStream2,
    rule_set: &'a model::RuleSet,
    _mode: PhantomData<M>,
}

#[derive(Clone, Copy)]
enum Binding<'a> {
    Ident(&'a Ident),
    Index(usize),
}

impl ToTokens for Binding<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Binding::Ident(v) => v.to_tokens(tokens),
            Binding::Index(v) => format_ident!("_{v}").to_tokens(tokens),
        }
    }
}

impl<M: ValidationMode> ToTokens for Rules<'_, M> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Rules {
            rules_mod,
            rule_set,
            _mode,
        } = self;

        let await_token = M::await_token();
        for custom_rule in rule_set.custom_rules.iter() {
            quote! {
                let point_1 = ();

                if let Err(__garde_error) = (#custom_rule)(&*__garde_binding, &__garde_user_ctx)#await_token {
                    __garde_report.append(__garde_path(), __garde_error);
                }
            }
            .to_tokens(tokens);
        }

        for rule in rule_set.rules.iter() {
            let name = TokenStream2::from_str(rule.name()).unwrap();
            use model::ValidateRule::*;
            let args = match rule {
                Ascii | Alphanumeric | Email | Url | CreditCard | PhoneNumber | Required => {
                    quote!(())
                }
                Ip => {
                    quote!((#rules_mod::ip::IpKind::Any,))
                }
                IpV4 => {
                    quote!((#rules_mod::ip::IpKind::V4,))
                }
                IpV6 => {
                    quote!((#rules_mod::ip::IpKind::V6,))
                }
                LengthSimple(range)
                | LengthBytes(range)
                | LengthChars(range)
                | LengthGraphemes(range)
                | LengthUtf16(range) => match range {
                    model::ValidateRange::GreaterThan(min) => {
                        quote!((#min, usize::MAX))
                    }
                    model::ValidateRange::LowerThan(max) => {
                        quote!((0usize, #max))
                    }
                    model::ValidateRange::Between(min, max) => {
                        quote!((#min, #max))
                    }
                    model::ValidateRange::Equal(equal) => {
                        quote!((#equal, #equal))
                    }
                },
                Matches(path) => {
                    quote!((stringify!(#path), &self.#path))
                }
                Range(range) => match range {
                    model::ValidateRange::GreaterThan(min) => quote!((Some(#min), None)),
                    model::ValidateRange::LowerThan(max) => quote!((None, Some(#max))),
                    model::ValidateRange::Between(min, max) => quote!((Some(#min), Some(#max))),
                    model::ValidateRange::Equal(equal) => quote!((Some(#equal), Some(#equal))),
                },
                Contains(expr) | Prefix(expr) | Suffix(expr) => {
                    quote_spanned!(expr.span() => (&#expr,))
                }
                Pattern(pat) => match pat {
                    model::ValidatePattern::Expr(expr) => quote_spanned!(expr.span() => (&#expr,)),
                    #[cfg(all(feature = "regex", feature = "js-sys"))]
                    model::ValidatePattern::Lit(s) => quote!({
                        #[cfg(not(all(
                            target_arch = "wasm32",
                            target_os = "unknown"
                        )))]
                        static PATTERN: #rules_mod::pattern::regex::StaticPattern =
                            #rules_mod::pattern::regex::init_pattern!(#s);

                        #[cfg(all(
                            target_arch = "wasm32",
                            target_os = "unknown"
                        ))]
                        static PATTERN: #rules_mod::pattern::regex_js_sys::StaticPattern =
                            #rules_mod::pattern::regex_js_sys::init_pattern!(#s);

                        (&PATTERN,)
                    }),
                    #[cfg(all(feature = "regex", not(feature = "js-sys")))]
                    model::ValidatePattern::Lit(s) => quote!({
                        static PATTERN: #rules_mod::pattern::regex::StaticPattern =
                            #rules_mod::pattern::regex::init_pattern!(#s);

                        (&PATTERN,)
                    }),
                },
            };

            //NOTE: all normal rules are sync, so no await_token is added
            quote! {
                let point_2 =();
                if let Err(__garde_error) = (#rules_mod::#name::apply)(&*__garde_binding, #args) {
                    __garde_report.append(__garde_path(), __garde_error);
                }
            }
            .to_tokens(tokens)
        }
    }
}

struct Fields<I, F, M: ValidationMode>(RefCell<Option<I>>, F, PhantomData<M>);

impl<'a, I, F, Extra, M> ToTokens for Fields<I, F, M>
where
    I: Iterator<Item = (Binding<'a>, &'a model::ValidateField, Extra)> + 'a,
    F: Fn(Extra, TokenStream2) -> TokenStream2,
    M: ValidationMode,
{
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let fields = match self.0.borrow_mut().take() {
            Some(v) => v,
            None => return,
        };

        let fields = fields.filter(|(_, field, _)| field.skip.is_none());
        let default_rules_mod = quote!(::garde::rules);

        let validate_trait = M::validate_path();
        let await_token = M::await_token();

        for (binding, field, extra) in fields {
            let field_adapter = field
                .adapter
                .as_ref()
                .map(|p| p.to_token_stream())
                .unwrap_or_default();

            let rules_mod = match field.adapter.as_ref() {
                Some(_) => &field_adapter,
                None => &default_rules_mod,
            };
            let rules = Rules::<M> {
                rules_mod,
                rule_set: &field.rule_set,
                _mode: PhantomData,
            };
            let outer = match field.has_top_level_rules() {
                true => Some(quote! {{#rules}}),
                false => None,
            };
            let inner = match (&field.dive, &field.rule_set.inner) {
                (Some((_, None)), None) => Some(quote! {
                    #validate_trait::validate_into(
                        &*__garde_binding,
                        __garde_user_ctx,
                        &mut __garde_path,
                        __garde_report,
                    )#await_token;
                }),
                (Some((_, Some(ctx))), None) => Some(quote! {
                    #validate_trait::validate_into(
                        &*__garde_binding,
                        &#ctx,
                        &mut __garde_path,
                        __garde_report,
                    )#await_token;
                }),
                (None, Some(inner)) => Some(
                    Inner::<M> {
                        rules_mod,
                        rule_set: inner,
                        _mode: PhantomData,
                    }
                    .to_token_stream(),
                ),
                (None, None) => None,
                // TODO: encode this via the type system instead?
                _ => unreachable!("`dive` and `inner` are mutually exclusive"),
            };

            let value = match (outer, inner) {
                (Some(outer), Some(inner)) => quote! {
                    let __garde_binding = &*#binding;
                    #inner
                    #outer
                },
                (None, Some(inner)) => quote! {
                    let __garde_binding = &*#binding;
                    #inner
                },
                (Some(outer), None) => quote! {
                    let __garde_binding = &*#binding;
                    #outer
                },
                (None, None) => unreachable!("field should already be skipped"),
            };

            let add = &self.1;

            add(extra, value).to_tokens(tokens)
        }
    }
}

struct Bindings<'a>(&'a model::ValidateVariant);

impl ToTokens for Bindings<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match &self.0 {
            model::ValidateVariant::Struct(fields) => {
                let names = fields
                    .iter()
                    .filter(|field| field.1.skip.is_none())
                    .map(|field| &field.0)
                    .collect::<Vec<_>>();
                let rest = if names.len() != fields.len() {
                    Some(quote!(..))
                } else {
                    None
                };

                quote!( { #(#names,)* #rest } )
            }
            model::ValidateVariant::Tuple(fields) => {
                let indices = fields
                    .iter()
                    .enumerate()
                    .filter(|(_, field)| field.skip.is_none())
                    .map(|(i, _)| IndexBinding(i))
                    .collect::<Vec<_>>();
                let rest = if indices.len() != fields.len() {
                    Some(quote!(..))
                } else {
                    None
                };

                quote!( ( #(#indices,)* #rest ) )
            }
        }
        .to_tokens(tokens)
    }
}

struct IndexBinding(usize);
impl ToTokens for IndexBinding {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        format_ident!("_{}", self.0).to_tokens(tokens)
    }
}
