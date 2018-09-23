#![recursion_limit = "128"]
#![feature(
    proc_macro_non_items,
    proc_macro_diagnostic,
    proc_macro_quote,
)]

extern crate eosio_sys;
extern crate eosio_types;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use eosio_sys::ctypes::CString;
use eosio_types::*;
use proc_macro::{Span, TokenStream};
use proc_macro2::TokenTree;
use std::str;
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Data, DeriveInput, Expr, ExprLit, Fields, FnArg, GenericParam, Generics, Ident, Index, ItemFn,
    Lit, LitInt, LitStr, Type,
};

#[proc_macro]
pub fn n(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();
    let input_str = input_string.as_str();
    let name_result = string_to_name(input_str);

    match name_result {
        Ok(name) => TokenStream::from(quote!(#name)),
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

struct SymbolInput {
    precision: LitInt,
    name: Ident,
}

impl Parse for SymbolInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let precision: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let name: Ident = input.parse()?;
        Ok(SymbolInput { precision, name })
    }
}

#[proc_macro]
pub fn s(input: TokenStream) -> TokenStream {
    let SymbolInput { precision, name } = parse_macro_input!(input as SymbolInput);
    let symbol_result = string_to_symbol(precision.value() as u8, &name.to_string());

    let expanded = match symbol_result {
        Ok(symbol) => quote!(#symbol),
        Err(error) => {
            let span = Span::call_site();
            let err = match error {
                ToSymbolError::IsEmpty => span
                    .error("symbol is empty")
                    .help("EOSIO symbols must be 1-12 characters long"),
                ToSymbolError::TooLong => span
                    .error("name is too long")
                    .help("EOSIO symbols must be 1-12 characters long"),
                ToSymbolError::BadChar(c) => {
                    let error_message = format!("name has bad character '{}'", c);
                    let help_message = "EOSIO symbols can only contain uppercase letters A-Z";
                    span.error(error_message).help(help_message)
                }
            };
            err.emit();
            quote!(0)
        }
    };

    TokenStream::from(quote!(#expanded))
}

#[proc_macro]
pub fn c(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();
    let input_str = input_string.as_str().trim_matches('"');
    let cstring = CString::new(input_str).unwrap();
    let bytes = cstring.to_bytes_with_nul();
    let c_str = str::from_utf8(bytes).unwrap();
    format!("\"{}\"", c_str).parse().unwrap()
}

#[proc_macro]
pub fn eosio_print(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
    let args = parser.parse(input).unwrap();
    let mut prints = quote!();
    for i in args.iter() {
        let mut printable = quote!(#i);
        if let Expr::Lit(ref lit) = *i {
            if let Lit::Str(ref strlit) = lit.lit {
                printable = quote!(c!(#strlit));
            }
        }
        prints = quote! {
            #prints
            #printable.print();
        };
    }
    TokenStream::from(quote!(#prints))
}

struct Assert {
    test: Expr,
    message: LitStr,
}

impl Parse for Assert {
    fn parse(input: ParseStream) -> Result<Self> {
        let test: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let message: LitStr = input.parse()?;
        Ok(Assert { test, message })
    }
}

#[proc_macro]
pub fn eosio_assert(input: TokenStream) -> TokenStream {
    let Assert { test, message } = parse_macro_input!(input as Assert);
    let expanded = quote! {
        unsafe {
            ::eosio_sys::eosio_assert(
                if #test { 1 } else { 0 },
                c!(#message).as_ptr()
            )
        }
    };
    TokenStream::from(quote!(#expanded))
}

#[proc_macro_attribute]
pub fn eosio_action(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let ident = input.ident;
    let decl = input.decl;
    let inputs = decl.inputs;
    let reads = inputs.iter().enumerate().map(|(i, f)| match f {
        FnArg::Captured(input) => {
            let pat = &input.pat;
            let ty = &input.ty;
            let read = quote_spanned! { ty.span() =>
                <#ty as ::eosio_bytes::Readable>::read(&bytes, pos)
            };
            quote! {
                let (#pat, pos) = match #read {
                    Ok(v) => v,
                    Err(_) => {
                        eosio_assert(false, c!("read"));
                        return
                    }
                };
            }
        }
        _ => unimplemented!(),
    });
    let block = input.block;
    let expanded = quote! {
        fn #ident() {
            // TODO: set the length of this to a fixed size based on the action inputs
            let mut bytes = [0u8; 10000];
            let ptr: *mut ::eosio_sys::c_void = &mut bytes[..] as *mut _ as *mut ::eosio_sys::c_void;
            unsafe {
                ::eosio_sys::read_action_data(
                    ptr,
                    ::eosio_sys::action_data_size()
                );
            }

            let pos = 0;
            #(#reads)*
            #block
        }
    };
    TokenStream::from(quote!(#expanded))
    // input
}

#[proc_macro]
pub fn eosio_abi(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_separated_nonempty;
    let args = parser.parse(input).unwrap();
    let mut actions = quote!();
    for i in args.iter() {
        actions = quote! {
            #actions
            n!(#i) => #i(),
        };
    }
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
            if action == n!(onerror) {
                eosio_assert(
                    code == n!(eosio),
                    c!("onerror action's are only valid from the \"eosio\" system account")
                );
            }
            if code == receiver || action == n!(onerror) {
                match action {
                    #actions
                    _ => {
                        eosio_assert(false, c!("bad action"));
                    }
                }
            }
        }
    };
    TokenStream::from(quote!(#expanded))
}

#[proc_macro]
pub fn wee_alloc(input: TokenStream) -> TokenStream {
    let expanded = quote! {
        #[cfg(not(test))]
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

        #[cfg(not(test))]
        #[panic_handler]
        #[no_mangle]
        pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
            unsafe {
                ::core::intrinsics::abort();
            }
        }

        #[cfg(not(test))]
        #[alloc_error_handler]
        #[no_mangle]
        pub extern "C" fn oom(_: ::core::alloc::Layout) -> ! {
            unsafe {
                ::core::intrinsics::abort();
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
