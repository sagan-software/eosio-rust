use crate::proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Ident};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let eosio = crate::paths::eosio();

    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#eosio));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let call_site = ::proc_macro2::Span::call_site();
    let reads = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_reads = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {f.span() =>
                        let #ident = <#ty as #eosio::Read>::read(bytes, pos)?;
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
                        let #ident = <#ty as #eosio::Read>::read(bytes, pos)?;
                    }
                });
                let fields_list = fields.unnamed.iter().enumerate().map(|(i, _f)| {
                    let ident = Ident::new(format!("field_{}", i).as_str(), call_site);
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
        impl #impl_generics #eosio::Read for #name #ty_generics #where_clause {
            #[inline]
            fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, #eosio::ReadError> {
                #reads
            }
        }
    };

    TokenStream::from(expanded)
}
