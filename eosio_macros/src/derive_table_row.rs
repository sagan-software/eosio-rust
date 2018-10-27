use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{Data, DeriveInput, Fields, GenericParam, Meta};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let eosio = crate::paths::eosio();

    let name = input.ident.clone();

    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#eosio::Read));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let table_name = input.attrs.iter().fold(None, |acc, attr| {
        match attr.interpret_meta() {
            Some(meta) => {
                let name = meta.name();
                if name == "table_name" {
                    if acc.is_some() {
                        panic!("only 1 table_name attribute allowed per struct");
                    }
                    match meta {
                        Meta::NameValue(meta) => {
                            let lit = meta.lit;
                            let s = Ident::new(format!("{}", quote!(#lit)).as_str().trim_matches('"'), Span::call_site());
                            Some(s)
                        }
                        _ => {
                            panic!("invalid table_name attribute. must be in the form #[table_name = \"test\"]");
                        }
                    }
                } else {
                    acc
                }
            }
            None => acc,
        }
    });

    if table_name.is_none() {
        panic!("#[table_name] attribute must be used when deriving from TableRow");
    }

    let expanded = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let mut primary_key = None;
                let mut secondary_keys = Vec::new();
                for field in fields.named.iter() {
                    for attr in field.attrs.iter() {
                        let name = attr.interpret_meta().map(|m| m.name());
                        let (is_primary, is_secondary) = name
                            .map(|n| (n == "primary", n == "secondary"))
                            .unwrap_or_else(|| (false, false));
                        if is_primary {
                            if primary_key.is_none() {
                                primary_key = field.ident.clone();
                            } else {
                                panic!("only 1 primary key allowed");
                            }
                        }

                        if is_secondary {
                            secondary_keys.push((field.ident.clone(), field.ty.clone()));
                        }
                    }
                }
                if primary_key.is_none() {
                    panic!("no primary key found");
                }
                if secondary_keys.len() > 16 {
                    panic!("up to 16 secondary keys are allowed");
                }

                let mut secondary_keys_expanded = quote!();
                let mut secondary_keys_constructors = quote!();
                for i in 0..16 {
                    match secondary_keys.get(i) {
                        Some((ident, ty)) => {
                            secondary_keys_expanded = quote! {
                                #secondary_keys_expanded
                                Some(&self.#ident),
                            };
                            secondary_keys_constructors = quote! {
                                #secondary_keys_constructors

                                pub fn #ident<C, S>(code: C, scope: S) -> #eosio::SecondaryTableIndex<#ty, Self>
                                where
                                    C: Into<#eosio::AccountName>,
                                    S: Into<#eosio::TableScope>,
                                {
                                    #eosio::SecondaryTableIndex::new(code, scope, #eosio::n!(#table_name), #ty::default(), #i)
                                }
                            };
                        }
                        None => {
                            secondary_keys_expanded = quote! {
                                #secondary_keys_expanded
                                None,
                            };
                        }
                    };
                }

                quote! {
                    #[automatically_derived]
                    impl #impl_generics #eosio::TableRow for #name #ty_generics #where_clause {
                        const NAME: u64 = #eosio::n!(#table_name);

                        fn primary_key(&self) -> u64 {
                            self.#primary_key.into()
                        }
                        fn secondary_keys(&self) -> [Option<&#eosio::SecondaryTableKey>; 16] {
                            [
                                #secondary_keys_expanded
                            ]
                        }
                    }

                    #[automatically_derived]
                    impl #impl_generics #name #ty_generics #where_clause {
                        #secondary_keys_constructors
                    }
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    TokenStream::from(expanded)
}
