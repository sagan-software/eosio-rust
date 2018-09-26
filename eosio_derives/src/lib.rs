#![recursion_limit = "128"]
#![feature(
    proc_macro_non_items,
    proc_macro_diagnostic,
    proc_macro_quote,
)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod derive_readable;
mod derive_table_row;
mod derive_writeable;

use proc_macro::TokenStream;

#[proc_macro_derive(Writeable, attributes(writeable_path))]
pub fn derive_writeable(input: TokenStream) -> TokenStream {
    ::derive_writeable::expand(input)
}

#[proc_macro_derive(Readable, attributes(readable_path))]
pub fn derive_readable(input: TokenStream) -> TokenStream {
    ::derive_readable::expand(input)
}

#[proc_macro_derive(TableRow, attributes(primary))]
pub fn derive_table_row(input: TokenStream) -> TokenStream {
    ::derive_table_row::expand(input)
}
