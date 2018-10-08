use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{FnArg, Ident, ItemFn};

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let ident = input.ident;
    let decl = input.decl;
    let inputs = decl.inputs;
    let vis = input.vis;
    // let reads = inputs.iter().map(|f| match f {
    //     FnArg::Captured(input) => {
    //         let pat = &input.pat;
    //         let ty = &input.ty;
    //         let read = quote_spanned! { ty.span() =>
    //             <#ty as ::eosio::bytes::Read>::read(&bytes, pos)
    //         };
    //         quote! {
    //             // TODO: Create a struct with #[derive(Read)] ?
    //             let (#pat, pos) = match #read {
    //                 Ok(v) => v,
    //                 Err(_) => {
    //                     ::eosio::macros::eosio_assert!(false, "read");
    //                     return
    //                 }
    //             };
    //         }
    //     }
    //     _ => unimplemented!(),
    // });
    let mut struct_fields = quote!();
    let mut assign_args = quote!();
    for input in inputs.iter() {
        match input {
            FnArg::Captured(input) => {
                let pat = &input.pat;
                let ty = &input.ty;
                struct_fields = quote! {
                    #struct_fields
                    pub #pat: #ty,
                };
                assign_args = quote! {
                    #assign_args
                    let #pat = s.#pat;
                };
            }
            _ => unimplemented!(),
        }
    }
    let block = input.block;

    let call_site = ::proc_macro2::Span::call_site();
    let struct_name = titlecase(ident.to_string().as_str());
    let struct_ident = Ident::new(format!("{}Args", struct_name).as_str(), call_site);

    let expanded = quote! {
        #[derive(Read, Write, Clone)]
        struct #struct_ident {
            #(#struct_fields)*
        }

        impl #struct_ident {
            fn action<'a>(
                self,
                authorization: &'a [::eosio::prelude::PermissionLevel],
            ) -> ::eosio::prelude::Action<'a, Self> {
                ::eosio::prelude::Action {
                    account: ::eosio::prelude::current_receiver(),
                    name: ::eosio::prelude::n!(#ident).into(),
                    authorization,
                    data: self,
                }
            }
        }

        // TODO: keep original function intact so it can be called like normal
        #vis fn #ident() {
            // TODO: set the length of this to a fixed size based on the action inputs
            let mut bytes = [0u8; 10000];
            let ptr: *mut ::eosio::sys::c_void = &mut bytes[..] as *mut _ as *mut ::eosio::sys::c_void;
            unsafe {
                ::eosio::sys::read_action_data(
                    ptr,
                    ::eosio::sys::action_data_size()
                );
            }

            let (s, _) = #struct_ident::read(&bytes, 0).assert("read");
            #(#assign_args)*
            #block
        }
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
