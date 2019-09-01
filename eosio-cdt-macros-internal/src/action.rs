use crate::proc_macro::TokenStream;
use eosio_core::NAME_UTF8_CHARS;
use heck::CamelCase;
use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ItemFn};

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let eosio_core = crate::paths::eosio_core();
    let ident = input.ident;
    let decl = input.decl;
    let inputs = decl.inputs;
    let vis = input.vis;
    let mut struct_fields = quote!();
    let mut assign_args = quote!();
    for input in inputs.iter() {
        match input {
            FnArg::Captured(input) => {
                let pat = &input.pat;
                let ty = &input.ty;
                let ty_str = quote!(#ty).to_string();
                let serde_attr = if ty_str == "bool" {
                    quote!(
                        #[cfg_attr(
                            feature = "serde",
                            serde(
                                deserialize_with = "::eosio::bool_from_u8",
                                serialize_with = "::eosio::bool_to_u8"
                            )
                        )]
                    )
                } else {
                    quote!()
                };
                struct_fields = quote! {
                    #struct_fields
                    #serde_attr
                    pub #pat: #ty,
                };
                assign_args = quote! {
                    #assign_args
                    let #pat = self.#pat;
                };
            }
            _ => unimplemented!(),
        }
    }
    let block = input.block;

    let call_site = ::proc_macro2::Span::call_site();
    let struct_ident = {
        let name = ident.to_string().as_str().to_camel_case();
        Ident::new(&name, call_site)
    };

    let action_ident = {
        let name = ident.to_string().as_str().to_camel_case().to_lowercase();
        Ident::new(&name, call_site)
    };

    let expanded = quote! {
        #[derive(Clone, #eosio_core::Read, #eosio_core::Write, #eosio_core::NumBytes)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct #struct_ident {
            #(#struct_fields)*
        }

        #[automatically_derived]
        impl #eosio_core::ToAction for #struct_ident {
            const NAME: u64 = #eosio_core::n!(#action_ident);
        }

        #[automatically_derived]
        impl #eosio_core::ActionFn for #struct_ident {
            fn execute(self) {
                #(#assign_args)*
                #block
            }
        }

        // TODO: keep original function intact so it can be called like normal
        #vis fn #ident() {
            let s = #struct_ident::read_data().expect("read");
            s.execute();
        }
    };
    TokenStream::from(quote!(#expanded))
    // input
}
