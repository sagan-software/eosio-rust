use crate::proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Index};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let eosio = crate::paths::eosio();

    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#eosio::NumBytes));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let call_site = ::proc_macro2::Span::call_site();
    let var = quote!(self);
    let add_to_count = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let access = quote_spanned!(call_site => #var.#name);
                    quote_spanned! { f.span() =>
                        count += #eosio::NumBytes::num_bytes(&#access);
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index {
                        index: i as u32,
                        span: call_site,
                    };
                    let access = quote_spanned!(call_site => #var.#index);
                    quote_spanned! { f.span() =>
                        count += #eosio::NumBytes::num_bytes(&#access);
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => {
                quote!{}
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics #eosio::NumBytes for #name #ty_generics #where_clause {
            fn num_bytes(&self) -> usize {
                let mut count = 0;
                #add_to_count
                count
            }
        }
    };

    TokenStream::from(expanded)
}
