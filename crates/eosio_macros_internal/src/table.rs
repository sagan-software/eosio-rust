use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::{DeriveInput, LitStr};

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
}

impl Parse for TableArgs {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = input.parse::<LitStr>()?;
        Ok(Self { name })
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
            #[eosio(table_name = #name)]
            #input
        };
        expanded.to_tokens(tokens);
    }
}
