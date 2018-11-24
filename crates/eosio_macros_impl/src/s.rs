use crate::proc_macro::{Span, TokenStream};
use eosio_sys::{string_to_symbol, ParseSymbolError};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, LitInt, Token};

struct SymbolInput {
    precision: LitInt,
    name: Ident,
}

impl Parse for SymbolInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let precision: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let name: Ident = input.parse()?;
        Ok(SymbolInput { precision, name })
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let SymbolInput { precision, name } = parse_macro_input!(input as SymbolInput);
    let symbol_result = string_to_symbol(precision.value() as u8, &name.to_string());

    let expanded = match symbol_result {
        Ok(symbol) => quote!(#symbol),
        Err(error) => {
            let span = Span::call_site();
            let err = match error {
                ParseSymbolError::IsEmpty => span
                    .error("symbol is empty")
                    .help("EOSIO symbols must be 1-12 characters long"),
                ParseSymbolError::TooLong => span
                    .error("name is too long")
                    .help("EOSIO symbols must be 1-12 characters long"),
                ParseSymbolError::BadChar(c) => {
                    let error_message = format!("name has bad character '{}'", c);
                    let help_message = "EOSIO symbols can only contain uppercase letters A-Z";
                    span.error(error_message).help(help_message)
                }
            };
            err.emit();
            quote!(0)
        }
    };

    TokenStream::from(quote!(#expanded))
}
