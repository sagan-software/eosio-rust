#![no_std]
#![feature(proc_macro_non_items, proc_macro_diagnostic, alloc)]

#[macro_use]
extern crate alloc;
extern crate eosio_sys;
extern crate eosio_types;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use alloc::prelude::*;
use core::str::{self, FromStr};
use eosio_sys::ctypes::CString;
use eosio_types::{string_to_name, ToNameError};
use proc_macro::{Span, TokenStream};

#[proc_macro]
pub fn n(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();
    let input_str = input_string.as_str();
    let name_result = string_to_name(input_str);

    match name_result {
        Ok(name) => TokenStream::from_str(&name.to_string()).unwrap(),
        Err(error) => {
            let span = Span::call_site();
            let err = match error {
                ToNameError::IsEmpty => span
                    .error("name is empty")
                    .help("EOSIO names must be 1-12 characters long"),
                ToNameError::TooLong => span
                    .error("name is too long")
                    .help("EOSIO names must be 1-12 characters long"),
                ToNameError::BadChar(c) => {
                    let error_message = format!("name has bad character '{}'", c);
                    let help_message =
                        "EOSIO names can only contain these characters: .12345abcdefghijklmnopqrstuvwxyz";
                    span.error(error_message).help(help_message)
                }
            };
            err.emit();
            "0".parse().unwrap()
        }
    }
}

#[proc_macro]
pub fn a(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn s(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn cstr(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();
    let input_str = input_string.as_str().trim_matches('"');
    let cstring = CString::new(input_str).unwrap();
    let bytes = cstring.to_bytes_with_nul();
    let c_str = str::from_utf8(bytes).unwrap();
    format!("\"{}\".as_ptr()", c_str).parse().unwrap()
}
