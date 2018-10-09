use eosio_sys::ctypes::CString;
use proc_macro::TokenStream;
use proc_macro2::Span;
use std::str;
use syn::LitStr;

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let input = input.value();
    let cstring = CString::new(input.as_str()).unwrap();
    let bytes = cstring.to_bytes_with_nul();
    let c_str = str::from_utf8(bytes).unwrap();
    let output = LitStr::new(c_str, Span::call_site());
    TokenStream::from(quote!(#output))
}
