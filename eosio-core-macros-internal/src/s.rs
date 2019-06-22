use crate::proc_macro::TokenStream;
use eosio_core::symbol_from_str;
use proc_macro2::{Literal, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, LitInt, Token};

struct EosioSymbol(u64);

impl Parse for EosioSymbol {
    fn parse(input: ParseStream) -> Result<Self> {
        let precision = input.parse::<LitInt>()?.value();
        input.parse::<Token![,]>()?;

        let mut code = String::new();
        while !input.is_empty() {
            let segment = input.fork().parse::<TokenTree>()?.to_string();
            input.parse::<TokenTree>()?;
            code += &segment;
        }

        let precision = match u8::try_from(precision) {
            Ok(p) => p,
            Err(_) => {
                return Err(input.error(format!(
                    "precision of {} is too large; must be <= {}",
                    precision,
                    u8::max_value()
                )))
            }
        };

        symbol_from_str(precision, code.as_str())
            .map(Self)
            .map_err(|e| input.error(e))
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
