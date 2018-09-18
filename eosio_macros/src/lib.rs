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
use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream, Parser, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Data, DeriveInput, Expr, ExprLit, Fields, FnArg, GenericParam, Generics, Ident, Index, ItemFn,
    Lit, LitStr, Type,
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
    format!("\"{}\"", c_str).parse().unwrap()
}

#[proc_macro]
pub fn print(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
    let args = parser.parse(input).unwrap();
    let mut prints = quote!();
    for i in args.iter() {
        let mut printable = quote!(#i);
        if let Expr::Lit(ref lit) = *i {
            if let Lit::Str(ref strlit) = lit.lit {
                printable = quote!(cstr!(#strlit));
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
            ::eosio::sys::system::eosio_assert(
                if #test { 1 } else { 0 },
                cstr!(#message).as_ptr()
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
                        reads = quote! {
                            #reads
                            let (#pat, count) = #ty_ident::read(&bytes[pos..]).unwrap();
                            pos += count;
                        };
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
            // TODO: set the length of this to a fixed size based on the action inputs
            let mut bytes = [0u8; 10000];
            let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
            unsafe {
                ::eosio::sys::action::read_action_data(
                    ptr,
                    ::eosio::sys::action::action_data_size()
                );
            }

            let mut pos = 0;
            #reads
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
                eosio_assert!(
                    code == n!(eosio),
                    "onerror action's are only valid from the \"eosio\" system account"
                );
            }
            if code == receiver || action == n!(onerror) {
                match action {
                    #actions
                    _ => {
                        eosio_assert!(false, "bad action");
                    }
                }
            }
        }
    };
    TokenStream::from(quote!(#expanded))
}

#[proc_macro_derive(Writeable)]
pub fn derive_writeable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::eosio::bytes::Writeable));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let call_site = ::proc_macro2::Span::call_site();
    let mut writes = quote!();
    let var = quote!(self);
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let access = quote_spanned!(call_site => #var.#name);
                    quote_spanned! {f.span() =>
                        pos += ::eosio::bytes::Writeable::write(&#access, &mut bytes[pos..])?;
                    }
                });
                writes = quote! {
                    let mut pos = 0;
                    #(#recurse)*
                    Ok(pos)
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index {
                        index: i as u32,
                        span: call_site,
                    };
                    let access = quote_spanned!(call_site => #var.#index);
                    quote_spanned! {f.span() =>
                        pos += ::eosio::bytes::Writeable::write(&#access, &mut bytes[pos..])?;
                    }
                });
                writes = quote! {
                    let mut pos = 0;
                    #(#recurse)*
                    Ok(pos)
                }
            }
            Fields::Unit => {
                writes = quote! {
                    Ok(0)
                };
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        impl #impl_generics ::eosio::bytes::Writeable for #name #ty_generics #where_clause {
            fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
                #writes
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Readable)]
pub fn derive_readable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::eosio::bytes::Readable));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let call_site = ::proc_macro2::Span::call_site();
    let mut reads = quote!();

    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_reads = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {f.span() =>
                        let (#ident, p) = <#ty as ::eosio::bytes::Readable>::read(&bytes[pos..])?;
                        pos += p;
                    }
                });
                let field_names = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    quote! {
                        #ident,
                    }
                });
                reads = quote! {
                    let mut pos = 0;
                    #(#field_reads)*
                    let item = #name {
                        #(#field_names)*
                    };
                    Ok((item, pos))
                };
            }
            Fields::Unnamed(ref fields) => {
                unimplemented!();
            }
            Fields::Unit => {
                unimplemented!();
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        impl #impl_generics ::eosio::bytes::Readable for #name #ty_generics #where_clause {
            fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
                #reads
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
