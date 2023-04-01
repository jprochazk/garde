use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};

use crate::model;

pub fn emit(input: model::Validate) -> TokenStream2 {
    input.to_token_stream()
}

impl ToTokens for model::Validate {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.ident;
        let context_ty = &self.context;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let kind = &self.kind;

        quote! {
            impl #impl_generics ::garde::Validate for #ident #ty_generics #where_clause {
                type Context = #context_ty ;

                fn validate(&self, __garde_user_ctx: &Self::Context) -> ::core::result::Result<(), ::garde::error::Errors> {
                    (
                        #kind
                    )
                    .finish()
                }
            }
        }
        .to_tokens(tokens)
    }
}

impl ToTokens for model::ValidateKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            model::ValidateKind::Struct(variant) => {
                let bindings = Bindings(variant);
                let validation = Validation(variant);

                quote! {{
                    let Self #bindings = self;
                    #validation
                }}
            }
            model::ValidateKind::Enum(variants) => {
                let variants = variants.iter().map(|(name, variant)| {
                    let bindings = Bindings(variant);
                    let validation = Validation(variant);

                    quote!(Self::#name #bindings => #validation)
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

struct Validation<'a>(&'a model::ValidateVariant);

impl<'a> ToTokens for Validation<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // TODO: deduplicate this a bit
        match &self.0 {
            model::ValidateVariant::Struct(fields) => {
                let fields =
                    fields
                        .iter()
                        .filter(|(_, field)| !field.skip.value)
                        .map(|(ident, field)| {
                            let key = ident.to_string();
                            let binding = Binding::Ident(ident);
                            let rules = Rules(binding, field);
                            if field.dive {
                                if field.has_top_level_rules() {
                                    quote! {
                                        __garde_errors.insert(
                                            #key,
                                            ::garde::error::Errors::nested(
                                                |__garde_errors| {#rules},
                                                ::garde::validate::Validate::validate(&#binding, __garde_user_ctx)
                                                    .err()
                                                    .unwrap_or_else(::garde::error::Errors::empty)
                                            )
                                        );
                                    }
                                } else {
                                    quote! {
                                        __garde_errors.insert(
                                            #key,
                                            ::garde::validate::Validate::validate(&#binding, __garde_user_ctx)
                                                .err()
                                                .unwrap_or_else(::garde::error::Errors::empty)
                                        );
                                    }
                                }
                            } else {
                                quote! {
                                    __garde_errors.insert(
                                        #key,
                                        ::garde::error::Errors::simple(|__garde_errors| {#rules})
                                    );
                                }
                            }
                        });

                quote! {
                    ::garde::error::Errors::fields(|__garde_errors| {#(#fields)*})
                }
            }
            model::ValidateVariant::Tuple(fields) => {
                let fields = fields
                    .iter()
                    .enumerate()
                    .filter(|(_, field)| !field.skip.value)
                    .map(|(i, field)| {
                        let binding = Binding::Index(i);
                        let rules = Rules(binding, field);
                        if field.dive {
                            if field.has_top_level_rules() {
                                quote! {
                                    __garde_errors.push(
                                        ::garde::error::Errors::nested(
                                            |__garde_errors| {#rules},
                                            ::garde::validate::Validate::validate(&#binding, __garde_user_ctx)
                                                .err()
                                                .unwrap_or_else(::garde::error::Errors::empty)
                                        )
                                    );
                                }
                            } else {
                                quote! {
                                    __garde_errors.push(
                                        ::garde::validate::Validate::validate(&#binding, __garde_user_ctx)
                                            .err()
                                            .unwrap_or_else(::garde::error::Errors::empty)
                                    );
                                }
                            }
                        } else {
                            quote! {
                                __garde_errors.push(
                                    ::garde::error::Errors::simple(|__garde_errors| {#rules})
                                );
                            }
                        }
                    });

                quote! {
                    ::garde::error::Errors::list(|__garde_errors| {#(#fields)*})
                }
            }
        }
        .to_tokens(tokens)
    }
}

struct Rules<'a>(Binding<'a>, &'a model::ValidateField);

#[derive(Clone, Copy)]
enum Binding<'a> {
    Ident(&'a Ident),
    Index(usize),
}

impl<'a> ToTokens for Binding<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Binding::Ident(v) => v.to_tokens(tokens),
            Binding::Index(v) => format_ident!("_{v}").to_tokens(tokens),
        }
    }
}

impl<'a> ToTokens for Rules<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let binding = &self.0;
        let ty = &self.1.ty;
        let custom_rules = self.1.custom_rules.iter().map(|func| {
            quote! {
                if let Err(__garde_error) = (#func)(&*#binding, &__garde_user_ctx) {
                    __garde_errors.push(__garde_error)
                }
            }
        });
        let rules = self.1.rules.iter().map(|rule| {
            assert!(rule.depth == 0);
            let name = format_ident!("{}", rule.name());
            let args = match &rule.kind {
                model::ValidateRuleKind::Ascii
                | model::ValidateRuleKind::Alphanumeric
                | model::ValidateRuleKind::Email
                | model::ValidateRuleKind::Url
                | model::ValidateRuleKind::CreditCard
                | model::ValidateRuleKind::PhoneNumber => quote!(()),
                model::ValidateRuleKind::Ip => {
                    quote!((::garde::rules::ip::IpKind::Any,))
                }
                model::ValidateRuleKind::IpV4 => {
                    quote!((::garde::rules::ip::IpKind::V4,))
                }
                model::ValidateRuleKind::IpV6 => {
                    quote!((::garde::rules::ip::IpKind::V6,))
                }
                model::ValidateRuleKind::Length(range)
                | model::ValidateRuleKind::ByteLength(range) => match range {
                    model::ValidateRange::GreaterThan(min) => quote!((#min, usize::MAX)),
                    model::ValidateRange::LowerThan(max) => quote!((0, #max)),
                    model::ValidateRange::Between(min, max) => quote!((#min, #max)),
                },
                model::ValidateRuleKind::Range(range) => match range {
                    model::ValidateRange::GreaterThan(min) => {
                        quote!((&#min, &(<#ty as ::garde::rules::range::Bounds>::MAX)))
                    }
                    model::ValidateRange::LowerThan(max) => {
                        quote!((&(<#ty as ::garde::rules::range::Bounds>::MIN), &#max))
                    }
                    model::ValidateRange::Between(min, max) => quote!((&#min, &#max)),
                },
                model::ValidateRuleKind::Contains(s)
                | model::ValidateRuleKind::Prefix(s)
                | model::ValidateRuleKind::Suffix(s) => quote!((#s,)),
                model::ValidateRuleKind::Pattern(s) => quote!({
                    static PATTERN: ::garde::rules::pattern::StaticPattern =
                        ::garde::rules::pattern::init_pattern!(#s);
                    (&PATTERN,)
                }),
            };
            quote! {
                if let Err(__garde_error) = (::garde::rules::#name::apply)(&*#binding, #args) {
                    __garde_errors.push(__garde_error)
                }
            }
        });

        quote! {
            #(#custom_rules)*
            #(#rules)*
        }
        .to_tokens(tokens)
    }
}

struct Bindings<'a>(&'a model::ValidateVariant);

impl<'a> ToTokens for Bindings<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match &self.0 {
            model::ValidateVariant::Struct(fields) => {
                let names = fields
                    .iter()
                    .filter(|field| !field.1.skip.value)
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
                    .filter(|(_, field)| !field.skip.value)
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
