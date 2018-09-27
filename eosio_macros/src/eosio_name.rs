use proc_macro::TokenStream;
use syn::Ident;

pub fn expand(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let expanded = quote! {
        #[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write)]
        #[eosio_internal]
        pub struct #ident(u64);

        impl From<u64> for #ident {
            fn from(n: u64) -> Self {
                #ident(n)
            }
        }

        impl Into<u64> for #ident {
            fn into(self) -> u64 {
                self.0
            }
        }

        impl ::print::Printable for #ident {
            fn print(&self) {
                unsafe { ::eosio::sys::printn((*self).into()) }
            }
        }
    };
    expanded.into()
}
