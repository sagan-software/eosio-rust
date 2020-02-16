use eosio_numstr::symbol_from_bytes;
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Result},
    LitInt, LitStr, Token,
};

pub struct EosioSymbol(u64);

impl Parse for EosioSymbol {
    fn parse(input: ParseStream) -> Result<Self> {
        let precision = input.parse::<LitInt>()?.base10_parse::<u8>()?;
        input.parse::<Token![,]>()?;
        let code = input.parse::<LitStr>()?.value();
        symbol_from_bytes(precision, code.bytes())
            .map(Self)
            .map_err(|e| input.error(e))
    }
}

impl ToTokens for EosioSymbol {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u64_suffixed(self.0))
    }
}
