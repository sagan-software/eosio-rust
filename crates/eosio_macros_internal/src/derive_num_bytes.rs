//! Derive `NumBytes`.
use crate::internal::get_root_path;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    parse_quote,
    spanned::Spanned,
    Data, DeriveInput, Fields, GenericParam, Generics, Ident, Index, Path,
};

pub struct DeriveNumBytes {
    ident: Ident,
    generics: Generics,
    data: Data,
    root_path: Path,
}

impl Parse for DeriveNumBytes {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let DeriveInput {
            attrs,
            ident,
            mut generics,
            data,
            ..
        } = input.parse()?;
        let root_path = get_root_path(&attrs);
        for param in &mut generics.params {
            if let GenericParam::Type(ref mut type_param) = *param {
                type_param.bounds.push(parse_quote!(#root_path::NumBytes));
            }
        }
        Ok(Self {
            ident,
            generics,
            data,
            root_path,
        })
    }
}

impl ToTokens for DeriveNumBytes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let (impl_generics, ty_generics, where_clause) =
            &self.generics.split_for_impl();
        let call_site = ::proc_macro2::Span::call_site();
        let var = quote!(self);
        let root = &self.root_path;
        let add_to_count = match &self.data {
            Data::Struct(ref data) => match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let access = quote_spanned!(call_site => #var.#name);
                        quote_spanned! { f.span() =>
                            count += #root::NumBytes::num_bytes(&#access);
                        }
                    });
                    quote! {
                        #(#recurse)*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse =
                        fields.unnamed.iter().enumerate().map(|(i, f)| {
                            let index = Index {
                                index: i as u32,
                                span: call_site,
                            };
                            let access =
                                quote_spanned!(call_site => #var.#index);
                            quote_spanned! { f.span() =>
                                count += #root::NumBytes::num_bytes(&#access);
                            }
                        });
                    quote! {
                        #(#recurse)*
                    }
                }
                Fields::Unit => {
                    quote! {}
                }
            },
            Data::Enum(_) | Data::Union(_) => unimplemented!(),
        };

        let expanded = quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl #impl_generics #root::NumBytes for #name #ty_generics #where_clause {
                #[inline]
                fn num_bytes(&self) -> usize {
                    let mut count = 0;
                    #add_to_count
                    count
                }
            }
        };
        expanded.to_tokens(tokens);
    }
}
