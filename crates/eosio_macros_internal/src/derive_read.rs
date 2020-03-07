//! Derive `Read`.
use crate::internal::get_root_path;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    parse_quote,
    spanned::Spanned,
    Data, DeriveInput, Fields, GenericParam, Generics, Ident, Path,
};

pub struct DeriveRead {
    ident: Ident,
    generics: Generics,
    data: Data,
    root_path: Path,
}

impl Parse for DeriveRead {
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
                type_param.bounds.push(parse_quote!(#root_path::Read));
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

impl ToTokens for DeriveRead {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.ident;
        let (impl_generics, ty_generics, where_clause) =
            &self.generics.split_for_impl();
        let root = &self.root_path;
        let call_site = ::proc_macro2::Span::call_site();
        let reads = match &self.data {
            Data::Struct(ref data) => match data.fields {
                Fields::Named(ref fields) => {
                    let field_reads = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {f.span() =>
                        let #ident = <#ty as #root::Read>::read(bytes, pos)?;
                    }
                });
                    let field_names = fields.named.iter().map(|f| {
                        let ident = &f.ident;
                        quote! {
                            #ident,
                        }
                    });
                    quote! {
                        #(#field_reads)*
                        let item = #name {
                            #(#field_names)*
                        };
                        Ok(item)
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let field_reads = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let ty = &f.ty;
                    let ident = Ident::new(format!("field_{}", i).as_str(), call_site);
                    quote_spanned! {f.span() =>
                        let #ident = <#ty as #root::Read>::read(bytes, pos)?;
                    }
                });
                    let fields_list =
                        fields.unnamed.iter().enumerate().map(|(i, _f)| {
                            let ident = Ident::new(
                                format!("field_{}", i).as_str(),
                                call_site,
                            );
                            quote! {
                                #ident,
                            }
                        });
                    quote! {
                        #(#field_reads)*
                        let item = #name(
                            #(#fields_list)*
                        );
                        Ok(item)
                    }
                }
                Fields::Unit => {
                    unimplemented!();
                }
            },
            Data::Enum(_) | Data::Union(_) => unimplemented!(),
        };

        let expanded = quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl #impl_generics #root::Read for #name #ty_generics #where_clause {
                #[inline]
                fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, #root::ReadError> {
                    #reads
                }
            }
        };
        expanded.to_tokens(tokens);
    }
}

// /// Expand input
// pub fn expand(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let root = crate::root_path(&input);

//     let name = input.ident;

//     let mut generics = input.generics;
//     for param in &mut generics.params {
//         if let GenericParam::Type(ref mut type_param) = *param {
//             type_param.bounds.push(parse_quote!(#root::Read));
//         }
//     }
//     let (impl_generics, ty_generics, where_clause) =
// generics.split_for_impl();

//     let call_site = ::proc_macro2::Span::call_site();
//     let reads = match input.data {
//         Data::Struct(ref data) => match data.fields {
//             Fields::Named(ref fields) => {
//                 let field_reads = fields.named.iter().map(|f| {
//                     let ident = &f.ident;
//                     let ty = &f.ty;
//                     quote_spanned! {f.span() =>
//                         let #ident = <#ty as #root::Read>::read(bytes, pos)?;
//                     }
//                 });
//                 let field_names = fields.named.iter().map(|f| {
//                     let ident = &f.ident;
//                     quote! {
//                         #ident,
//                     }
//                 });
//                 quote! {
//                     #(#field_reads)*
//                     let item = #name {
//                         #(#field_names)*
//                     };
//                     Ok(item)
//                 }
//             }
//             Fields::Unnamed(ref fields) => {
//                 let field_reads = fields.unnamed.iter().enumerate().map(|(i,
// f)| {                     let ty = &f.ty;
//                     let ident = Ident::new(format!("field_{}", i).as_str(),
// call_site);                     quote_spanned! {f.span() =>
//                         let #ident = <#ty as #root::Read>::read(bytes, pos)?;
//                     }
//                 });
//                 let fields_list =
//                     fields.unnamed.iter().enumerate().map(|(i, _f)| {
//                         let ident = Ident::new(
//                             format!("field_{}", i).as_str(),
//                             call_site,
//                         );
//                         quote! {
//                             #ident,
//                         }
//                     });
//                 quote! {
//                     #(#field_reads)*
//                     let item = #name(
//                         #(#fields_list)*
//                     );
//                     Ok(item)
//                 }
//             }
//             Fields::Unit => {
//                 unimplemented!();
//             }
//         },
//         Data::Enum(_) | Data::Union(_) => unimplemented!(),
//     };

//     let expanded = quote! {
//         #[automatically_derived]
//         #[allow(unused_qualifications)]
//         impl #impl_generics #root::Read for #name #ty_generics #where_clause
// {             #[inline]
//             fn read(bytes: &[u8], pos: &mut usize) -> Result<Self,
// #root::ReadError> {                 #reads
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }
