//! Internal use only.
//!
//! This crate is an implementation detail that will hopefully go away
//! once the [`proc_macro_hygiene`] feature is stabilized. In the meantime
//! we must use this crate (and [`proc_macro_hack`]) to allow for
//! function-like procedural macros in expression positions.
//!
//! [`proc_macro_hygiene`]: https://doc.rust-lang.org/beta/unstable-book/language-features/proc-macro-hygiene.html
//! [`proc_macro_hack`]: https://github.com/dtolnay/proc-macro-hack
//!
#![allow(
    clippy::unimplemented,
    clippy::missing_inline_in_public_items,
    clippy::missing_docs_in_private_items
)]
extern crate proc_macro;

mod n;
mod s;

use crate::proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn n(input: TokenStream) -> TokenStream {
    crate::n::expand(input)
}

#[proc_macro_hack]
pub fn s(input: TokenStream) -> TokenStream {
    crate::s::expand(input)
}
