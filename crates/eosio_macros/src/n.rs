use crate::proc_macro::{Span, TokenStream};
use eosio_sys::{string_to_name, ParseNameError, NAME_CHARS};
use std::str;
use syn::Expr;

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);
    let input_string = format!("{}", quote!(#input));
    let input_str = input_string.as_str().replace(' ', "");
    let name_result = string_to_name(&input_str);

    match name_result {
        Ok(name) => TokenStream::from(quote!(#name)),
        Err(error) => {
            let span = Span::call_site();
            let err = match error {
                ParseNameError::IsEmpty => span
                    .error("name is empty")
                    .help("EOSIO names must be 1-12 characters long"),
                ParseNameError::TooLong => span
                    .error("name is too long")
                    .help("EOSIO names must be 1-12 characters long"),
                ParseNameError::BadChar(c) => {
                    let error_message = format!("name has bad character '{}'", c);
                    let help_message = format!(
                        "EOSIO names can only contain these characters: {}",
                        str::from_utf8(&NAME_CHARS).unwrap()
                    );
                    span.error(error_message).help(help_message)
                }
            };
            err.emit();
            "0".parse().unwrap()
        }
    }
}
