use crate::proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, GenericParam, Index};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let eosio = crate::paths::eosio();

    let name = input.ident;

    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#eosio::Write));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let call_site = ::proc_macro2::Span::call_site();
    let var = quote!(self);
    let writes = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let access = quote_spanned!(call_site => #var.#name);
                    quote_spanned! { f.span() =>
                        let pos = #eosio::Write::write(&#access, bytes, pos)?;
                    }
                });
                quote! {
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
                        let pos = #eosio::Write::write(&#access, bytes, pos)?;
                    }
                });
                quote! {
                    #(#recurse)*
                    Ok(pos)
                }
            }
            Fields::Unit => {
                quote! {
                    Ok(pos)
                }
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics #eosio::Write for #name #ty_generics #where_clause {
            fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, #eosio::WriteError> {
                #writes
            }
        }
    };

    TokenStream::from(expanded)
}
