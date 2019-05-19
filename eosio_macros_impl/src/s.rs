use crate::proc_macro::TokenStream;
use eosio_cdt_sys::{string_to_symbol, ParseSymbolError};
use proc_macro2::{Literal, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitInt, Token};

struct EosioSymbol(u64);

fn accept_char_in_symbol_code(ch: char) -> bool {
    ch >= 'A' && ch <= 'Z'
}

impl Parse for EosioSymbol {
    fn parse(input: ParseStream) -> Result<Self> {
        let precision = input.parse::<LitInt>()?.value();
        input.parse::<Token![,]>()?;

        let mut code = String::new();
        while !input.is_empty() {
            let segment = input.fork().parse::<TokenTree>()?.to_string();
            if !segment.chars().all(accept_char_in_symbol_code) {
                break;
            }
            input.parse::<TokenTree>()?;
            code += &segment;
        }

        string_to_symbol(precision as u8, code.as_str())
            .map(EosioSymbol)
            .map_err(|e| {
                let message = match e {
                    ParseSymbolError::IsEmpty =>
                        "symbol is empty. EOSIO symbols must be 1-7 characters long".to_string(),
                    ParseSymbolError::TooLong =>
                        "symbol is too long. EOSIO symbols must be 7 characters or less".to_string(),
                    ParseSymbolError::BadChar(c) =>
                        format!("symbol has bad character '{}'. EOSIO symbols can only contain uppercase letters A-Z", c),
                };
                input.error(message)
            })
    }
}

impl ToTokens for EosioSymbol {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        tokens.append(Literal::u64_suffixed(self.0))
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let symbol = parse_macro_input!(input as EosioSymbol);
    quote!(#symbol).into()
}
