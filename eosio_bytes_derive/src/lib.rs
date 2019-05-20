extern crate proc_macro;

mod derive_num_bytes;
mod derive_read;
mod derive_write;

use crate::proc_macro::TokenStream;
use cfg_if::cfg_if;
use quote::quote;

cfg_if! {
    if #[cfg(feature = "internal-use-only-root-path-is-crate")] {
        pub(crate) fn root_path() -> ::proc_macro2::TokenStream { quote!(crate) }
    } else if #[cfg(feature = "internal-use-only-root-path-is-eosio")] {
        pub(crate) fn root_path() -> ::proc_macro2::TokenStream { quote!(::eosio) }
    } else {
        pub(crate) fn root_path() -> ::proc_macro2::TokenStream { quote!(::eosio_bytes) }
    }
}

#[proc_macro_derive(Write)]
pub fn derive_write(input: TokenStream) -> TokenStream {
    crate::derive_write::expand(input)
}

#[proc_macro_derive(Read)]
pub fn derive_read(input: TokenStream) -> TokenStream {
    crate::derive_read::expand(input)
}

#[proc_macro_derive(NumBytes)]
pub fn derive_num_bytes(input: TokenStream) -> TokenStream {
    crate::derive_num_bytes::expand(input)
}
