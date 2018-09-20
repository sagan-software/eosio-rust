#![recursion_limit = "128"]
#![feature(
    proc_macro_non_items,
    proc_macro_diagnostic,
    proc_macro_quote,
)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, GenericParam, Ident, Index};

#[proc_macro_derive(Writeable)]
pub fn derive_writeable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::eosio_bytes::Writeable));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let call_site = ::proc_macro2::Span::call_site();
    let mut writes = quote!();
    let var = quote!(self);
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let access = quote_spanned!(call_site => #var.#name);
                    quote_spanned! {f.span() =>
                        pos += ::eosio_bytes::Writeable::write(&#access, &mut bytes[pos..])?;
                    }
                });
                writes = quote! {
                    let mut pos = 0;
                    #(#recurse)*
                    Ok(pos)
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index {
                        index: i as u32,
                        span: call_site,
                    };
                    let access = quote_spanned!(call_site => #var.#index);
                    quote_spanned! {f.span() =>
                        pos += ::eosio_bytes::Writeable::write(&#access, &mut bytes[pos..])?;
                    }
                });
                writes = quote! {
                    let mut pos = 0;
                    #(#recurse)*
                    Ok(pos)
                }
            }
            Fields::Unit => {
                writes = quote! {
                    Ok(0)
                };
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        impl #impl_generics ::eosio_bytes::Writeable for #name #ty_generics #where_clause {
            fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
                #writes
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Readable)]
pub fn derive_readable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::eosio_bytes::Readable));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let call_site = ::proc_macro2::Span::call_site();
    let mut reads = quote!();

    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_reads = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {f.span() =>
                        let (#ident, p) = <#ty as ::eosio_bytes::Readable>::read(&bytes[pos..])?;
                        pos += p;
                    }
                });
                let field_names = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    quote! {
                        #ident,
                    }
                });
                reads = quote! {
                    let mut pos = 0;
                    #(#field_reads)*
                    let item = #name {
                        #(#field_names)*
                    };
                    Ok((item, pos))
                };
            }
            Fields::Unnamed(ref fields) => {
                let field_reads = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let ty = &f.ty;
                    let ident = Ident::new(format!("field_{}", i).as_str(), call_site);
                    quote_spanned! {f.span() =>
                        let (#ident, p) = <#ty as ::eosio_bytes::Readable>::read(&bytes[pos..])?;
                        pos += p;
                    }
                });
                let fields_list = fields.unnamed.iter().enumerate().map(|(i, _f)| {
                    let ident = Ident::new(format!("field_{}", i).as_str(), call_site);
                    quote! {
                        #ident,
                    }
                });
                reads = quote! {
                    let mut pos = 0;
                    #(#field_reads)*
                    let item = #name(
                        #(#fields_list)*
                    );
                    Ok((item, pos))
                };
            }
            Fields::Unit => {
                unimplemented!();
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        impl #impl_generics ::eosio_bytes::Readable for #name #ty_generics #where_clause {
            fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
                #reads
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(TableRow, attributes(primary))]
pub fn derive_table_row(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.clone();

    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::eosio_bytes::Readable));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let mut primary_key = None;
                for field in fields.named.iter() {
                    for attr in field.attrs.iter() {
                        let meta = attr.interpret_meta().map(|m| m.name() == "primary");
                        match (primary_key.is_none(), meta) {
                            (true, Some(true)) => primary_key = field.ident.clone(),
                            (false, Some(true)) => panic!("only 1 primary key allowed"),
                            _ => continue,
                        }
                    }
                }
                if primary_key.is_none() {
                    panic!("no primary key found");
                }
                quote! {
                    impl #impl_generics ::eosio::db::TableRow for #name #ty_generics #where_clause {
                        fn primary_key(&self) -> u64 {
                            self.#primary_key.as_u64()
                        }
                    }
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    proc_macro::TokenStream::from(expanded)
}
