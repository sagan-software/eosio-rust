use crate::proc_macro::TokenStream;
use eosio_sys::{string_to_name, ParseNameError, NAME_CHARS};
use proc_macro2::{Literal, TokenTree};
use quote::quote;
use quote::{ToTokens, TokenStreamExt};
use std::str;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input;

pub struct EosioName(u64);

fn accept_char_in_name(ch: char) -> bool {
    ch >= 'a' && ch <= 'z' || ch >= '1' && ch <= '5' || ch == '.'
}

impl Parse for EosioName {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut username = String::new();
        while !input.is_empty() {
            let segment = input.fork().parse::<TokenTree>()?.to_string();
            if !segment.chars().all(accept_char_in_name) {
                break;
            }
            input.parse::<TokenTree>()?;
            username += &segment;
        }

        string_to_name(username.as_str())
            .map(EosioName)
            .map_err(|e| {
                let message = match e {
                    ParseNameError::IsEmpty => "expected EOSIO name".to_string(),
                    ParseNameError::TooLong => {
                        format!("unexpected input; EOSIO name is {} characters long but must be 12 characters or less", username.len())
                    },
                    ParseNameError::BadChar(c) => {
                        format!("unexpected character '{}'; EOSIO names can only contain these characters: {}", c, str::from_utf8(&NAME_CHARS).unwrap())
                    }
                };
                input.error(message)
            })
    }
}

impl ToTokens for EosioName {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        tokens.append(Literal::u64_suffixed(self.0))
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as EosioName);
    quote!(#name).into()
}
