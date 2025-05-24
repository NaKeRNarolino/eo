use proc_macro2::Ident;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::Parse;
use syn::Type;

pub struct EventMacro {
    name: Ident,
    ty: Type
}

impl Parse for EventMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let ty: Type = input.parse()?;

        Ok(Self { name, ty })
    }
}

impl ToTokens for EventMacro {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let EventMacro { name, ty } = self;

        tokens.append_all(quote! {
            let #name: eo::events::Event<#ty> = eo::events::Event::new(RwLock::new(Vec::new()));
        })
    }
}


pub struct EventInitMacro {
    ty: Type
}

impl Parse for EventInitMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;

        Ok(Self { ty })
    }
}

impl ToTokens for EventInitMacro {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let EventInitMacro { ty } = self;

        tokens.append_all(quote! {
            eo::events::Event::<#ty>::new(RwLock::new(Vec::new()))
        })
    }
}