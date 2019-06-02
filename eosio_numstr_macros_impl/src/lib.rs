//! Implementation
//!
extern crate proc_macro;

mod n;
mod s;

use crate::proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

///
#[proc_macro_hack]
pub fn n(input: TokenStream) -> TokenStream {
    crate::n::expand(input)
}

#[proc_macro_hack]
pub fn s(input: TokenStream) -> TokenStream {
    crate::s::expand(input)
}
