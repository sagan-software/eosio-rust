use eosio_numstr::name_from_bytes;
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    LitStr,
};

pub struct EosioName(u64);

impl Parse for EosioName {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = input.parse::<LitStr>()?.value();
        name_from_bytes(name.bytes())
            .map(Self)
            .map_err(|e| input.error(e))
    }
}

impl ToTokens for EosioName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u64_suffixed(self.0))
    }
}
