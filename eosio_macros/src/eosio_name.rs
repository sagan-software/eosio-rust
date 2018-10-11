use proc_macro::TokenStream;
use syn::Ident;

pub fn expand(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let expanded = quote! {
        #[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Read, Write, Hash, PartialOrd, Ord)]
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

        // TODO: no_std
        impl std::str::FromStr for #ident {
            type Err = ::eosio_sys::ParseNameError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let name = ::eosio_sys::string_to_name(s)?;
                Ok(name.into())
            }
        }

        impl ::eosio::Print for #ident {
            fn print(&self) {
                unsafe { ::eosio::printn(self.0) }
            }
        }

        impl #ident {
            fn to_string(&self) -> String {
                unsafe { ::eosio::name_to_string(self.0) }
            }
        }
    };
    expanded.into()
}
