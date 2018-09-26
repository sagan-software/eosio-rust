use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Fields, GenericParam, Index, Meta, Path};

fn get_writeable_path(attrs: &[Attribute]) -> Path {
    let mut path: Path = parse_quote!(::eosio::writeable);
    for attr in attrs {
        if let Some(meta) = attr.interpret_meta() {
            match meta {
                Meta::Word(word) => {
                    if word == "eosio_internal" {
                        path = parse_quote!(::writeable);
                    }
                }
                _ => continue,
            }
        }
    }
    path
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let writeable_path = get_writeable_path(&input.attrs);

    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(#writeable_path::Writeable));
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
                    quote_spanned! { f.span() =>
                        let pos = #writeable_path::Writeable::write(&#access, bytes, pos)?;
                    }
                });
                writes = quote! {
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
                    quote_spanned! { f.span() =>
                        let pos = #writeable_path::Writeable::write(&#access, bytes, pos)?;
                    }
                });
                writes = quote! {
                    #(#recurse)*
                    Ok(pos)
                }
            }
            Fields::Unit => {
                writes = quote! {
                    Ok(pos)
                };
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        impl #impl_generics #writeable_path::Writeable for #name #ty_generics #where_clause {
            fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, #writeable_path::WriteError> {
                #writes
            }
        }
    };

    TokenStream::from(expanded)
}
