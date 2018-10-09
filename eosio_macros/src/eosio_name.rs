use proc_macro::TokenStream;
use syn::Ident;

pub fn expand(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let expanded = quote! {
        #[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write)]
        pub struct #ident(u64);

        impl From<u64> for #ident {
            fn from(n: u64) -> Self {
                #ident(n)
            }
        }

        impl From<#ident> for u64 {
            fn from(i: #ident) -> Self {
                i.0
            }
        }

        impl ::eosio::Printable for #ident {
            fn print(&self) {
                unsafe { ::eosio::printn((*self).into()) }
            }
        }
    };
    expanded.into()
}
