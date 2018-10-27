use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{DeriveInput, LitStr};

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let _eosio = crate::paths::eosio();
    let name = parse_macro_input!(args as Ident);
    let name = LitStr::new(format!("{}", quote!(#name)).as_str(), Span::call_site());
    let expanded = quote! {
        #[derive(Debug, TableRow, Read, Write, Clone, PartialEq, PartialOrd)]
        #[table_name = #name]
        #input
    };
    TokenStream::from(expanded)
    // input
}
