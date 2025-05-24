mod notifier_macros;
mod event_macro;

use proc_macro2::TokenTree;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, Expr, Token};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::event_macro::{EventInitMacro, EventMacro};
use crate::notifier_macros::{NotifierCreation, ReactiveValueCreation};

/// Helper macro to create [Notifier]s in a cleaner way.
/// Examples:
/// ```rust
/// use macros::notifier;
///
/// notifier!(a = 0);
/// notifier!(a i32 = 0);
/// ```
#[proc_macro]
pub fn notifier(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = parse_macro_input!(token_stream as NotifierCreation);

    quote! { #parsed }.into()
}

/// Helper macro to create [ReactiveValue]s in a cleaner way.
/// Examples:
///```rust
///use macros::{ notifier, reactive_value };
///
///notifier!(a = 0);
///
///reactive_value!(b = $a + 1); // prefixing a to make it a part of the reactive expression
///
///b.get() // will return a value based on the `a` notifier.
///
#[proc_macro]
pub fn reactive_value(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = TokenStream2::new();

    let mut p = proc_macro2::TokenStream::from(token_stream.clone()).into_iter().peekable();

    while let Some(t) = p.next() {
        if let TokenTree::Punct(punct) = &t {
            if (punct.as_char() == '$') {
                if let Some(TokenTree::Ident(ident)) = p.next() {
                    let ident = ident.to_string();
                    let ident = Ident::new(&ident, Span::call_site());
                    let rep = quote! { (&#ident).get() };

                    input.extend(rep);

                    continue;
                }
            }
        }

        input.extend([t]);
    }

    let e = proc_macro::TokenStream::from(input);
    let parsed = parse_macro_input!(e as ReactiveValueCreation);

    quote! { #parsed }.into()
}

/// A macro for easier creation of events
///```rust
///use macros::event;
///event!(event i32);
///```
#[proc_macro]
pub fn event(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let p = parse_macro_input!(token_stream as EventMacro);

    quote! { #p }.into()
}

/// A macro for easier initialization of events
///```rust
///use macros::event;
///event!(event i32);
///```
#[proc_macro]
pub fn event_init(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let p = parse_macro_input!(token_stream as EventInitMacro);

    quote! { #p }.into()
}


#[proc_macro]
pub fn infix(tks: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let infixes = parse_macro_input!(tks as InfixCollection);

    quote! { #infixes }.into()
}

struct InfixStatement {
    object: Ident,
    method: Ident,
    expr: Expr
}

struct InfixCollection {
    infixes: Punctuated<InfixStatement, Token![;]>,
}

impl Parse for InfixCollection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let infixes = Punctuated::<InfixStatement, Token![;]>::parse_terminated(input)?;

        Ok(Self { infixes })
    }
}

impl ToTokens for InfixCollection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let infixes = &self.infixes.iter().collect::<Vec<&InfixStatement>>();
        tokens.append_all(quote! {
            #(#infixes)*
        })
    }
}

impl Parse for InfixStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let object: Ident = input.parse()?;
        let method: Ident = input.parse()?;
        let expr: Expr = input.parse()?;

        Ok(InfixStatement {
            object, method, expr
        })
    }
}

impl ToTokens for InfixStatement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let InfixStatement { object, method, expr } = self;
        tokens.append_all(
            quote! {
                #object.#method(#expr);
            }
        )
    }
}