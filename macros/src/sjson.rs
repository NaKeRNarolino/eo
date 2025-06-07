use proc_macro2::{Delimiter, Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, bracketed, Token};

pub struct SJsonMacro {
    jsons: Vec<SJsonElement>
}

#[derive(Clone)]
struct SJsonElement {
    id: String,
    value: SJsonValue
}


#[derive(Clone)]
enum SJsonValue {
    String(String),
    Number(f64),
    Object(Vec<SJsonElement>),
    Array(Vec<SJsonValue>),
    Variable(Ident),
    Bool(bool)
}

impl Parse for SJsonMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            jsons: Punctuated::<SJsonElement, Token![,]>::parse_terminated(input)?.into_iter().collect::<Vec<SJsonElement>>()
        })
    }
}

impl Parse for SJsonElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name_punct =
            Punctuated::<Ident, Token![:]>::parse_separated_nonempty(input)?
                .into_iter().map(|x| x.to_string()).collect::<Vec<String>>();

        let name = name_punct.join(":");

        if input.peek2(syn::LitStr) || input.peek2(syn::LitFloat) ||
            input.peek2(syn::LitInt) || input.peek2(syn::LitBool) || input.peek2(Token![$]) {
            input.parse::<Token![=]>()?;
        }

        let value = input.parse::<SJsonValue>()?;

        Ok(SJsonElement {
            id: name,
            value
        })
    }
}

impl Parse for SJsonValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitStr) {
            Ok(Self::String(input.parse::<syn::LitStr>()?.value()))
        } else if input.peek(syn::LitInt) {
            Ok(Self::Number(input.parse::<syn::LitInt>()?.base10_parse::<f64>()?))
        } else if input.peek(syn::LitFloat) {
            Ok(Self::Number(input.parse::<syn::LitFloat>()?.base10_parse::<f64>()?))
        } else if input.peek(syn::LitBool) {
            Ok(Self::Bool(input.parse::<syn::LitBool>()?.value()))
        } else if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;

            Ok(Self::Variable(input.parse::<Ident>()?))
        } else if let Some((_, d, _, _)) = input.cursor().any_group() {
            if (d == Delimiter::Brace) {
                let contents;
                braced!(contents in input);

                let t: Punctuated<SJsonElement, Token![,]> = contents.parse_terminated(SJsonElement::parse, Token![,])?;


                let p = t.into_iter().collect::<Vec<SJsonElement>>();

                Ok(Self::Object(p))
            } else if (d == Delimiter::Bracket) {
                let contents;
                bracketed!(contents in input);

                let t: Punctuated<SJsonValue, Token![,]> = contents.parse_terminated(SJsonValue::parse, Token![,])?;

                let p = t.into_iter().collect::<Vec<SJsonValue>>();

                Ok(Self::Array(p))
            } else {
                panic!("Not a valid sJSON syntax.")
            }
        } else {
            panic!("Not a valid sJSON syntax.")
        }
    }
}

impl ToTokens for SJsonValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(
            match &self {
                SJsonValue::String(v) => {
                    quote! { eo::sjson::SJsonValue::String(#v.to_string()) }
                },
                SJsonValue::Number(v) => {
                    quote! { eo::sjson::SJsonValue::Number(#v) }
                },
                SJsonValue::Object(v) => {
                    let convert: Vec<SJsonElementHashMap> = v.into_iter().map(|x| SJsonElementHashMap::from(x.clone())).collect();

                    quote! {
                        eo::sjson::SJsonValue::Object(::std::collections::HashMap::from([
                            #(#convert),*
                        ]))
                    }
                }
                SJsonValue::Array(v) => {
                    quote! {
                        eo::sjson::SJsonValue::Array(::std::vec::Vec::<eo::sjson::SJsonValue>::from([
                            #(#v),*
                        ]))
                    }
                },
                SJsonValue::Variable(ident) => quote! { #ident.sjson() },
                SJsonValue::Bool(v) => quote! { eo::sjson::SJsonValue::Boolean(#v) },
            }
        )
    }
}

struct SJsonElementHashMap {
    id: String,
    value: SJsonValue
}

impl ToTokens for SJsonElementHashMap {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SJsonElementHashMap { id, value } = self;
        tokens.append_all(
            quote! {
                (#id.to_string(), #value)
            }
        )
    }
}

impl From<SJsonElement> for SJsonElementHashMap {
    fn from(value: SJsonElement) -> Self {
        Self {
            value: value.value,
            id: value.id
        }
    }
}

impl ToTokens for SJsonElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let SJsonElement { id, value } = self;
        tokens.append_all(quote! {
            eo::sjson::SJsonElement {
                id: #id.to_string(),
                params: #value
            }
        })
    }
}

impl ToTokens for SJsonMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = &self.jsons;
        tokens.append_all(
            quote! {
                std::vec::Vec::<eo::sjson::SJsonElement>::from([
                    #(#q),*
                ])
            }
        )
    }
}

pub struct SimplifiedSJsonMacro {
    value: SJsonValue,
}

impl Parse for SimplifiedSJsonMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { value: input.parse::<SJsonValue>()? })
    }
}

impl ToTokens for SimplifiedSJsonMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let v = &self.value;
        tokens.append_all(quote! { #v });
    }
}