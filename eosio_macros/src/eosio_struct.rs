use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{DeriveInput, LitStr};

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {
        #[derive(Read, Write, Clone, PartialEq, PartialOrd)]
        #input
    };
    TokenStream::from(expanded)
}
