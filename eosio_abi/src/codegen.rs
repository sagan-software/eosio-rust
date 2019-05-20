use heck::{CamelCase, SnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub struct Code(TokenStream);

impl ToString for Code {
    fn to_string(&self) -> String {
        // TODO: not working in stable rust
        // let mut out: Vec<u8> = Vec::new();
        // let mut config = rustfmt_nightly::Config::default();
        // config.set().emit_mode(rustfmt_nightly::EmitMode::Stdout);

        // {
        //     let mut session = rustfmt_nightly::Session::<Vec<u8>>::new(config, Some(&mut out));
        //     let input = rustfmt_nightly::Input::Text(self.0.to_string());
        //     session.format(input).unwrap();
        // }

        // String::from_utf8(out).unwrap()
        self.0.to_string()
    }
}

// https://github.com/EOSIO/eos/blob/master/libraries/chain/abi_serializer.cpp#L65-L103
const TYPES_MAP: &[(&str, &str)] = &[
    ("bool", "bool"),
    ("int8", "i8"),
    ("uint8", "u8"),
    ("int16", "i16"),
    ("uint16", "u16"),
    ("int32", "i32"),
    ("uint32", "u32"),
    ("int64", "i64"),
    ("uint64", "u64"),
    ("int128", "i128"),
    ("uint128", "u128"),
    ("varint32", "i32"),
    ("varuint32", "u32"),
    ("float32", "f32"),
    ("float64", "f64"),
    ("float128", "f128"),
    ("time_point", "::eosio::TimePoint"),
    ("time_point_sec", "::eosio::TimePointSec"),
    ("block_timestamp_type", "::eosio::BlockTimestamp"),
    ("name", "::eosio::Name"),
    ("bytes", "Vec<u8>"),
    ("string", "String"),
    ("checksum160", "::eosio::Checksum160"),
    ("checksum256", "::eosio::Checksum256"),
    ("checksum512", "::eosio::Checksum512"),
    ("public_key", "::eosio::PublicKey"),
    ("signature", "::eosio::Signature"),
    ("symbol", "::eosio::Symbol"),
    ("symbol_code", "::eosio::SymbolCode"),
    ("asset", "::eosio::Asset"),
    ("extended_asset", "::eosio::ExtendedAsset"),
];

fn to_type(s: &str) -> syn::Type {
    let new_t = match TYPES_MAP.iter().find(|t| s == t.0) {
        Some(t) => t.1.to_string(),
        None => s.to_camel_case(),
    };
    syn::parse_str::<syn::Type>(new_t.as_str()).unwrap()
}

impl crate::Type {
    fn to_tokens(&self) -> TokenStream {
        let span = Span::call_site();
        let type_ident =
            Ident::new(self.new_type_name.to_camel_case().as_str(), span);
        let from_type = to_type(self.type_.as_str());
        quote! {
            pub type #type_ident = #from_type;
        }
    }
}

impl crate::Field {
    fn to_tokens(&self) -> TokenStream {
        let mut name_snake = self.name.to_snake_case();
        let name_orig = self.name.as_str();
        let ident = match syn::parse_str::<syn::Ident>(name_snake.as_str()) {
            Ok(i) => i,
            Err(_) => {
                name_snake.push_str("_");
                syn::parse_str::<syn::Ident>(name_snake.as_str()).unwrap()
            }
        };
        let type_ = to_type(self.type_.as_str());
        if name_snake == name_orig {
            quote!(pub #ident: #type_,)
        } else {
            quote! {
                #[serde(rename = #name_orig)]
                pub #ident: #type_,
            }
        }
    }
}

impl crate::Abi {
    fn to_tokens(&self) -> TokenStream {
        let span = Span::call_site();
        let types = self.types.iter().map(|t| t.to_tokens());

        let structs = self.structs.iter().map(|s| {
            let struct_ident = Ident::new(s.name.to_camel_case().as_str(), span);
            let struct_fields = s.fields.iter().map(|f| f.to_tokens());
            let mut tokens = quote! {
                #[derive(::serde::Serialize, ::serde::Deserialize, Debug, Clone)]
                pub struct #struct_ident {
                    #(#struct_fields)*
                }
            };

            self.actions
                .iter()
                .filter(|a| a.type_ == s.name)
                .for_each(|a| {
                    let action_name = ::eosio::sys::string_to_name(&a.name).unwrap();
                    tokens = quote! {
                        #tokens

                        impl ::eosio::ToAction for #struct_ident {
                            const NAME: u64 = #action_name;
                        }
                    }
                });

            self.tables
                .iter()
                .filter(|t| t.type_ == s.name)
                .for_each(|t| {
                    let table_name = ::eosio::sys::string_to_name(&t.name).unwrap();
                    tokens = quote! {
                        #tokens

                        impl ::eosio::TableRow for #struct_ident {
                            const TABLE_NAME: u64 = #table_name;

                            fn primary_key(&self) -> u64 {
                                0
                            }
                        }
                    };
                });

            tokens
        });
        quote! {
            #(#types)*

            #(#structs)*
        }
    }

    pub fn to_code(&self) -> Code {
        Code(self.to_tokens())
    }
}
