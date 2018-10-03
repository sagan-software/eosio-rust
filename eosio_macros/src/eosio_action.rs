use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{FnArg, ItemFn};

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let ident = input.ident;
    let decl = input.decl;
    let inputs = decl.inputs;
    let vis = input.vis;
    let reads = inputs.iter().map(|f| match f {
        FnArg::Captured(input) => {
            let pat = &input.pat;
            let ty = &input.ty;
            let read = quote_spanned! { ty.span() =>
                <#ty as ::eosio::bytes::Read>::read(&bytes, pos)
            };
            quote! {
                // TODO: Create a struct with #[derive(Read)] ?
                let (#pat, pos) = match #read {
                    Ok(v) => v,
                    Err(_) => {
                        ::eosio::macros::eosio_assert!(false, "read");
                        return
                    }
                };
            }
        }
        _ => unimplemented!(),
    });
    let block = input.block;

    let expanded = quote! {
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

            let pos = 0;
            #(#reads)*
            let size = unsafe {::eosio::sys::action_data_size()};
            #block
        }
    };
    TokenStream::from(quote!(#expanded))
    // input
}
