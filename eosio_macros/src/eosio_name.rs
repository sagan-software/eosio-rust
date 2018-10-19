use proc_macro::TokenStream;
use syn::Ident;

pub fn expand(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let expanded = quote! {
        #[derive(Read, Write, Debug, PartialEq, Eq, Clone, Copy, Default, Hash, PartialOrd, Ord, ::serde_derive::Serialize, ::serde_derive::Deserialize)]
        pub struct #ident(u64);

        impl From<u64> for #ident {
            fn from(n: u64) -> Self {
                #ident(n)
            }
        }

        impl From<#ident> for u64 {
            fn from(i: #ident) -> Self {
                i.0
            }
        }

        // TODO: no_std
        impl std::str::FromStr for #ident {
            type Err = ::eosio::ParseNameError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let name = ::eosio::sys::string_to_name(s)?;
                Ok(name.into())
            }
        }

        impl ::eosio::Print for #ident {
            fn print(&self) {
                unsafe { ::eosio::sys::printn(self.0) }
            }
        }

        impl From<#ident> for String {
            fn from(i: #ident) -> Self {
                i.to_string()
            }
        }

        // TODO: no_std
        impl ::std::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let s = unsafe { ::eosio::sys::name_to_string(self.0) };
                write!(f, "{}", s)
            }
        }

        impl ::eosio::SecondaryTableKey for #ident {
            fn end(
                &self,
                code: ::eosio::AccountName,
                scope: ::eosio::TableScope,
                table: ::eosio::SecondaryTableName
            ) -> i32 {
                u64::from(*self).end(code, scope, table)
            }
            fn next(&self, iterator: i32) -> (i32, u64) {
                u64::from(*self).next(iterator)
            }
            fn previous(&self, iterator: i32) -> (i32, u64) {
                u64::from(*self).previous(iterator)
            }
            fn remove(&self, iterator: i32) {
                u64::from(*self).remove(iterator)
            }
            fn store(
                &self,
                scope: ::eosio::TableScope,
                table: ::eosio::SecondaryTableName,
                payer: ::eosio::AccountName,
                id: u64,
            ) -> i32 {
                u64::from(*self).store(scope, table, payer, id)
            }
            fn update(&self, iterator: i32, payer: AccountName) {
                u64::from(*self).update(iterator, payer)
            }
            fn lower_bound(
                &self,
                code: ::eosio::AccountName,
                scope: ::eosio::TableScope,
                table: ::eosio::SecondaryTableName,
            ) -> (i32, u64) {
                u64::from(*self).lower_bound(code, scope, table)
            }
            fn upper_bound(
                &self,
                code: ::eosio::AccountName,
                scope: ::eosio::TableScope,
                table: ::eosio::SecondaryTableName,
            ) -> (i32, u64) {
                u64::from(*self).upper_bound(code, scope, table)
            }
            fn find_primary(
                &self,
                code: ::eosio::AccountName,
                scope: ::eosio::TableScope,
                table: ::eosio::SecondaryTableName,
                primary: u64,
            ) -> i32 {
                u64::from(*self).find_primary(code, scope, table, primary)
            }
            fn find_secondary(
                &self,
                code: ::eosio::AccountName,
                scope: ::eosio::TableScope,
                table: ::eosio::SecondaryTableName,
            ) -> (i32, u64) {
                u64::from(*self).find_secondary(code, scope, table)
            }
        }
    };
    expanded.into()
}
