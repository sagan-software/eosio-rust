use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{Attribute, DeriveInput, LitStr};

pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = parse_macro_input!(args as Ident);
    let name = LitStr::new(format!("{}", quote!(#name)).as_str(), Span::call_site());
    let expanded = quote! {
        #[derive(TableRow, Read, Write)]
        #[table_name = #name]
        #input
    };
    TokenStream::from(expanded)
    // input
}
