#![recursion_limit = "128"]
#![feature(
    proc_macro_non_items,
    proc_macro_diagnostic,
    proc_macro_quote,
    alloc
)]

#[macro_use]
extern crate alloc;
extern crate core;
extern crate eosio_sys;
extern crate eosio_types;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use alloc::prelude::*;
use core::str::{self, FromStr};
use eosio_sys::ctypes::CString;
use eosio_types::{string_to_name, ToNameError, NAME_CHARS};
use proc_macro::{Span, TokenStream};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Expr, FnArg, ItemFn, Pat, PathSegment, Type};

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

#[proc_macro]
pub fn print(tokens: TokenStream) -> TokenStream {
    // let parser = Punctuated::<PathSegment, Token![,]>::parse_separated_nonempty;
    // let input = parser.parse(tokens).unwrap();
    tokens
}

#[proc_macro]
pub fn assert(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn action(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    // let attrs = input.attrs;
    // let vis = input.vis;
    // let constness = input.constness;
    // let unsafety = input.unsafety;
    // let asyncness = input.asyncness;
    // let abi = input.abi;
    let ident = input.ident;
    let decl = input.decl;
    let inputs = decl.inputs;
    let mut bytes_len: usize = 0;
    let mut reads = quote!();
    for input in inputs.iter() {
        match input {
            FnArg::Captured(input) => {
                let pat = &input.pat;
                let ty = &input.ty;
                match ty {
                    Type::Path(ty) => {
                        let segment = ty.path.segments.iter().next().unwrap();
                        let ty_ident = &segment.ident;
                        if ty_ident == "u64" {
                            let bytes_ends = bytes_len + 8;
                            reads = quote! {
                                #reads
                                let #pat = ::eosio::datastream::read_name(&bytes[#bytes_len..#bytes_ends]).unwrap();
                            };
                            bytes_len += 8;
                        }
                        println!("!!!, {}", bytes_len);
                    }
                    _ => println!("7"),
                }
            }
            _ => println!("NOT CAPTURED"),
        }
    }
    let block = input.block;
    let expanded = quote! {
        fn #ident() {
            let mut bytes = [0u8; #bytes_len];
            let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
            unsafe {
                ::eosio::sys::action::read_action_data(
                    ptr,
                    ::eosio::sys::action::action_data_size()
                );
            }
            #reads
            #block
        }
    };
    TokenStream::from(quote!(#expanded))
    // input
}
