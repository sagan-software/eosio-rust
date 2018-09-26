use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, GenericParam};

pub fn expand(input: TokenStream) -> TokenStream {
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

    TokenStream::from(expanded)
}
