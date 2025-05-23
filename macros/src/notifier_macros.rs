use std::net::Shutdown::Read;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Expr, Token};
use syn::parse::{Parse, ParseStream};
use syn::Type;

pub struct NotifierCreation {
    name: Ident,
    maybe_type: Option<Type>,
    initial_value: Expr,
}

impl Parse for NotifierCreation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let infer: bool = input.peek(Token![=]);

        let mut maybe_type: Option<Type> = None;

        if !infer {
            maybe_type = Some(input.parse()?);
        }

        input.parse::<Token![=]>()?;

        let initial_value: Expr = input.parse()?;

        Ok(NotifierCreation { name, maybe_type, initial_value })
    }
}

impl ToTokens for NotifierCreation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let NotifierCreation { name, maybe_type, initial_value } = self;

        tokens.append_all(match maybe_type {
            None => {
                quote! {
                    let mut #name = eo::notifiers::Notifier::new(#initial_value);
                }
            }
            Some(ty) => {
                quote! {
                    let mut #name: eo::notifiers::Notifier<#ty> = eo::notifiers::Notifier::new(#initial_value);
                }
            }
        })
    }
}

pub struct ReactiveValueCreation {
    name: Ident,
    maybe_type: Option<Type>,
    computer: Expr
}

impl Parse for ReactiveValueCreation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let infer: bool = input.peek(Token![=]);

        let mut maybe_type: Option<Type> = None;

        if !infer {
            maybe_type = Some(input.parse()?);
        }

        input.parse::<Token![=]>()?;

        let computer: Expr = input.parse()?;

        Ok(ReactiveValueCreation { name, maybe_type, computer })
    }
}

impl ToTokens for ReactiveValueCreation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ReactiveValueCreation { name, maybe_type, computer } = self;

        tokens.append_all(match maybe_type {
            None => {
                quote! {
                    let mut #name = eo::notifiers::ReactiveValue::new(::std::sync::Arc::new(|| #computer));
                }
            }
            Some(ty) => {
                quote! {
                    let mut #name: eo::notifiers::ReactiveValue<#ty> = eo::notifiers::Notifier::new(::std::sync::Arc::new(|| #computer));
                }
            }
        })
    }
}