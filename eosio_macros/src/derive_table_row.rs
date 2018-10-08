use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, GenericParam};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.clone();

    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::eosio::bytes::Read));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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

                                pub fn #ident<C, S, N>(code: C, scope: S, table: N) -> SecondaryIndex<#ty, Self>
                                where
                                    C: Into<AccountName>,
                                    S: Into<ScopeName>,
                                    N: Into<TableName>,
                                {
                                    SecondaryIndex::new(code, scope, table, #ty::default(), #i)
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
                    impl #impl_generics ::eosio::db::TableRow for #name #ty_generics #where_clause {
                        fn primary_key(&self) -> u64 {
                            self.#primary_key.into()
                        }
                        fn secondary_keys(&self) -> [Option<&SecondaryKey>; 16] {
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
