use crate::proc_macro::TokenStream;
use syn::{FnArg, Ident, ItemFn};

#[cfg(feature = "contract")]
pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let eosio = crate::paths::eosio();
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
                                deserialize_with = "::eosio::json::bool_from_u8",
                                serialize_with = "::eosio::json::bool_to_u8"
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
    let struct_name = titlecase(ident.to_string().as_str());
    let struct_ident = Ident::new(format!("{}Action", struct_name).as_str(), call_site);

    let expanded = quote! {
        #[derive(Clone, #eosio::Read, #eosio::Write, #eosio::NumBytes)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct #struct_ident {
            #(#struct_fields)*
        }

        #[automatically_derived]
        impl #eosio::ActionFn for #struct_ident {
            const NAME: u64 = n!(#ident);

            fn execute(self) {
                #(#assign_args)*
                #block
            }
        }

        // TODO: keep original function intact so it can be called like normal
        #vis fn #ident() {
            let (s, _) = #struct_ident::read_data().assert("read");
            s.execute();
        }
    };
    TokenStream::from(quote!(#expanded))
    // input
}

#[cfg(not(feature = "contract"))]
pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let eosio = crate::paths::eosio();
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
                                deserialize_with = "::eosio::json::bool_from_u8",
                                serialize_with = "::eosio::json::bool_to_u8"
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
            }
            _ => unimplemented!(),
        }
    }

    let call_site = ::proc_macro2::Span::call_site();
    let struct_name = titlecase(ident.to_string().as_str());
    let struct_ident = Ident::new(format!("{}Action", struct_name).as_str(), call_site);

    let expanded = quote! {
        #[derive(Clone, #eosio::Read, #eosio::Write, #eosio::NumBytes, Default)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct #struct_ident {
            #(#struct_fields)*
        }

        #[automatically_derived]
        impl #eosio::ActionFn for #struct_ident {
            const NAME: u64 = n!(#ident);
        }

        #vis fn #ident() { }
    };
    TokenStream::from(quote!(#expanded))
    // input
}

fn titlecase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
