#![recursion_limit = "512"]
// #![feature(proc_macro_hygiene, proc_macro_diagnostic, proc_macro_quote)]
// #![feature(proc_macro_diagnostic)]

extern crate proc_macro;

mod abi;
mod action;
mod print;
mod table;

mod paths {
    #[cfg(feature = "internal-use-only-root-path-is-eosio")]
    pub fn eosio_core() -> ::proc_macro2::TokenStream {
        ::quote::quote!(::eosio)
    }

    #[cfg(not(feature = "internal-use-only-root-path-is-eosio"))]
    pub fn eosio_core() -> ::proc_macro2::TokenStream {
        ::quote::quote!(::eosio_core)
    }

    #[cfg(feature = "internal-use-only-root-path-is-eosio")]
    pub fn eosio_cdt() -> ::proc_macro2::TokenStream {
        ::quote::quote!(::eosio)
    }

    #[cfg(not(feature = "internal-use-only-root-path-is-eosio"))]
    pub fn eosio_cdt() -> ::proc_macro2::TokenStream {
        ::quote::quote!(::eosio_cdt)
    }
}

use crate::proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

#[proc_macro]
pub fn abi(input: TokenStream) -> TokenStream {
    crate::abi::expand(input)
}

#[proc_macro_attribute]
pub fn action(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::action::expand(args, input)
}

#[proc_macro_hack]
pub fn print(input: TokenStream) -> TokenStream {
    crate::print::expand(input)
}

#[proc_macro_attribute]
pub fn table(args: TokenStream, input: TokenStream) -> TokenStream {
    crate::table::expand(args, input)
}
