use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result as ParseResult},
    token::Comma,
    DeriveInput, Error, Ident, LitStr,
};

pub struct Table {
    input: DeriveInput,
    args: TableArgs,
}

impl Table {
    pub const fn new(args: TableArgs, input: DeriveInput) -> Self {
        Self { args, input }
    }
}

pub struct TableArgs {
    name: LitStr,
    is_singleton: bool,
}

impl Parse for TableArgs {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = input.parse::<LitStr>()?;

        if input.parse::<Comma>().is_ok() {
            let ident = input.parse::<Ident>()?;
            if ident == "singleton" {
                Ok(Self {
                    name,
                    is_singleton: true,
                })
            } else {
                Err(Error::new(ident.span(), "expected `singleton`"))
            }
        } else {
            Ok(Self {
                name,
                is_singleton: false,
            })
        }
    }
}

impl ToTokens for Table {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let input = &self.input;
        let name = &self.args.name;
        let expanded = quote! {
            #[derive(
                Clone,
                Debug,
                eosio::NumBytes,
                eosio::Read,
                eosio::Write,
                eosio::Table,
                PartialEq,
                PartialOrd
            )]
        };
        let expanded = if self.args.is_singleton {
            quote! {
                #expanded
                #[eosio(table_name = #name, singleton)]
            }
        } else {
            quote! {
                #expanded
                #[eosio(table_name = #name)]
            }
        };
        let expanded = quote! {
            #expanded
            #input
        };
        expanded.to_tokens(tokens);
    }
}
