use eosio_sys::ctypes::CString;
use proc_macro::TokenStream;
use std::str;

pub fn expand(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();
    let input_str = input_string.as_str().trim_matches('"');
    let cstring = CString::new(input_str).unwrap();
    let bytes = cstring.to_bytes_with_nul();
    let c_str = str::from_utf8(bytes).unwrap();
    format!("\"{}\"", c_str).parse().unwrap()
}
