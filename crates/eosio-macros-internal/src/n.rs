use crate::proc_macro::TokenStream;
use eosio_numstr::name_from_str;
use proc_macro2::{Literal, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input;

pub struct EosioName(u64);

impl Parse for EosioName {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut username = String::new();
        while !input.is_empty() {
            let segment = input.fork().parse::<TokenTree>()?.to_string();
            input.parse::<TokenTree>()?;
            username += &segment;
        }

        name_from_str(username.as_str())
            .map(Self)
            .map_err(|e| input.error(e))
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
