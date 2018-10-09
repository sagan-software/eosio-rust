use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{FnArg, Ident, ItemFn};

pub fn expand(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
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
                struct_fields = quote! {
                    #struct_fields
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
        #[derive(Read, Write, Clone)]
        struct #struct_ident {
            #(#struct_fields)*
        }

        #[automatically_derived]
        impl ::eosio::Action for #struct_ident {
            const NAME: u64 = n!(#ident);

            fn execute(self) {
                #(#assign_args)*
                #block
            }
        }

        // TODO: keep original function intact so it can be called like normal
        #vis fn #ident() {
            let (s, _) = #struct_ident::read_action_data().assert("read");
            s.execute();
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
