use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput};

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {
        #[derive(TableRow, Read, Write)]
        #input
    };
    TokenStream::from(expanded)
    // input
}
