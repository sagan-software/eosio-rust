use crate::proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Expr, Token};

pub fn expand(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
    let eosio_cdt = crate::paths::eosio_cdt();
    let args = parser.parse(input).unwrap();
    let mut prints = quote!();
    for i in args.iter() {
        prints = quote! {
            #prints
            #eosio_cdt::Print::print(&#i);
        };
    }
    TokenStream::from(quote!(#prints))
}
