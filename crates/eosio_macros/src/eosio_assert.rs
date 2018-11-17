use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, LitStr};

struct Assert {
    test: Expr,
    message: LitStr,
}

impl Parse for Assert {
    fn parse(input: ParseStream) -> Result<Self> {
        let test: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let message: LitStr = input.parse()?;
        Ok(Assert { test, message })
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let Assert { test, message } = parse_macro_input!(input as Assert);
    let eosio = crate::paths::eosio();
    let expanded = quote! {
        unsafe {
            #eosio::eosio_assert(
                #test,
                #eosio::c!(#message)
            )
        }
    };
    TokenStream::from(quote!(#expanded))
}
