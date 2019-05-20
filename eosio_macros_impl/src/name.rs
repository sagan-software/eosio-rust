use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};

#[cfg(not(feature = "contract"))]
pub fn expand(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let eosio = crate::paths::eosio();

    let call_site = ::proc_macro2::Span::call_site();
    let identstr = ident.to_string();
    let identlitstr = LitStr::new(identstr.as_str(), call_site);
    let identvisitor =
        Ident::new(format!("{}Visitor", identstr).as_str(), call_site);

    let scope_name_converters = if identstr == "ScopeName" {
        quote!()
    } else {
        quote! {
            #[automatically_derived]
            impl From<#eosio::ScopeName> for #ident {
                #[inline]
                fn from(scope: #eosio::ScopeName) -> Self {
                    let value: u64 = scope.into();
                    value.into()
                }
            }

            #[automatically_derived]
            impl From<#ident> for #eosio::ScopeName {
                #[inline]
                fn from(name: #ident) -> Self {
                    let value: u64 = name.into();
                    value.into()
                }
            }
        }
    };

    let expanded = quote! {
        #[derive(#eosio::Read, #eosio::Write, #eosio::NumBytes, Debug, PartialEq, Eq, Clone, Copy, Default, Hash, PartialOrd, Ord)]
        pub struct #ident(pub u64);

        #[automatically_derived]
        impl From<u64> for #ident {
            #[inline]
            fn from(n: u64) -> Self {
                #ident(n)
            }
        }

        #[automatically_derived]
        impl From<#ident> for u64 {
            #[inline]
            fn from(i: #ident) -> Self {
                i.0
            }
        }

        #scope_name_converters

        // TODO: no_std
        #[automatically_derived]
        impl std::str::FromStr for #ident {
            type Err = #eosio::ParseNameError;
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let name = #eosio::sys::string_to_name(s)?;
                Ok(name.into())
            }
        }

        #[automatically_derived]
        impl std::convert::TryFrom<&str> for #ident {
            type Error = #eosio::ParseNameError;
            #[inline]
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::from_str(value)
            }
        }

        #[automatically_derived]
        impl std::convert::TryFrom<String> for #ident {
            type Error = #eosio::ParseNameError;
            #[inline]
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::from_str(value.as_str())
            }
        }

        // TODO: no_std
        #[automatically_derived]
        impl std::fmt::Display for #ident {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let s = unsafe { #eosio::sys::name_to_string(self.0) };
                write!(f, "{}", s)
            }
        }

        #[automatically_derived]
        impl From<#ident> for String {
            #[inline]
            fn from(i: #ident) -> Self {
                i.to_string()
            }
        }

        #[automatically_derived]
        impl #ident {
            // TODO: TryFrom
            #[inline]
            pub fn from_string(value: String) -> Result<Self, #eosio::ParseNameError> {
                Self::from_str(value.as_str())
            }

            // TODO: TryFrom
            #[inline]
            pub fn from_str(value: &str) -> Result<Self, #eosio::ParseNameError> {
                let name = #eosio::sys::string_to_name(value)?;
                Ok(name.into())
            }

            #[inline]
            pub const fn as_u64(&self) -> u64 {
                self.0
            }

            #[inline]
            pub const fn from_u64(value: u64) -> Self {
                #ident(value)
            }
        }

        #[automatically_derived]
        impl PartialEq<#ident> for String {
            #[inline]
            fn eq(&self, other: &#ident) -> bool {
                self.as_str() == other.to_string().as_str()
            }
        }

        #[automatically_derived]
        impl PartialEq<String> for #ident {
            #[inline]
            fn eq(&self, other: &String) -> bool {
                self.to_string().as_str() == other.as_str()
            }
        }

        #[cfg(feature = "serde")]
        struct #identvisitor;

        #[cfg(feature = "serde")]
        impl<'de> ::serde::de::Visitor<'de> for #identvisitor {
            type Value = #ident;

            #[inline]
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("an EOSIO name as a string or a number")
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(#ident::from(value))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                #ident::from_str(value).map_err(::serde::de::Error::custom)
            }

            #[inline]
            fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                #ident::from_str(value).map_err(::serde::de::Error::custom)
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                #ident::from_string(value).map_err(::serde::de::Error::custom)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> ::serde::de::Deserialize<'de> for #ident {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                deserializer.deserialize_any(
                    #identvisitor
                )
            }
        }

        // TODO: allow serializing to u64 somehow?
        #[cfg(feature = "serde")]
        impl ::serde::ser::Serialize for #ident {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.to_string().as_str())
            }
        }

        #[cfg(feature = "stdweb")]
        js_serializable!(#ident);
        #[cfg(feature = "stdweb")]
        js_deserializable!(#ident);

    };
    expanded.into()
}

#[cfg(feature = "contract")]
pub fn expand(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    let eosio = crate::paths::eosio();
    let identstr = ident.to_string();

    let scope_name_converters = if identstr == "ScopeName" {
        quote!()
    } else {
        quote! {
            #[automatically_derived]
            impl From<#eosio::ScopeName> for #ident {
                #[inline]
                fn from(scope: #eosio::ScopeName) -> Self {
                    let value: u64 = scope.into();
                    value.into()
                }
            }

            #[automatically_derived]
            impl From<#ident> for #eosio::ScopeName {
                #[inline]
                fn from(name: #ident) -> Self {
                    let value: u64 = name.into();
                    value.into()
                }
            }
        }
    };

    let expanded = quote! {
        #[derive(#eosio::Read, #eosio::Write, #eosio::NumBytes, Debug, PartialEq, Eq, Clone, Copy, Default, Hash, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        pub struct #ident(u64);

        #[automatically_derived]
        impl From<u64> for #ident {
            #[inline]
            fn from(n: u64) -> Self {
                #ident(n)
            }
        }

        #[automatically_derived]
        impl From<#ident> for u64 {
            #[inline]
            fn from(i: #ident) -> Self {
                i.0
            }
        }

        #scope_name_converters

        // TODO: no_std
        #[automatically_derived]
        impl std::str::FromStr for #ident {
            type Err = #eosio::ParseNameError;
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let name = #eosio::sys::string_to_name(s)?;
                Ok(name.into())
            }
        }

        #[automatically_derived]
        impl std::convert::TryFrom<&str> for #ident {
            type Error = #eosio::ParseNameError;
            #[inline]
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::from_str(value)
            }
        }

        #[automatically_derived]
        impl std::convert::TryFrom<String> for #ident {
            type Error = #eosio::ParseNameError;
            #[inline]
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::from_str(value.as_str())
            }
        }

        // TODO: no_std
        #[automatically_derived]
        impl std::fmt::Display for #ident {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let s = unsafe { #eosio::sys::name_to_string(self.0) };
                write!(f, "{}", s)
            }
        }

        #[automatically_derived]
        impl #eosio::Print for #ident {
            #[inline]
            fn print(&self) {
                unsafe { #eosio::sys::printn(self.0) }
            }
        }

        #[automatically_derived]
        impl From<#ident> for String {
            #[inline]
            fn from(i: #ident) -> Self {
                i.to_string()
            }
        }

        impl #ident {
            #[inline]
            pub fn from_string(value: String) -> Result<Self, #eosio::ParseNameError> {
                Self::from_str(value.as_str())
            }

            #[inline]
            pub fn from_str(value: &str) -> Result<Self, #eosio::ParseNameError> {
                let name = #eosio::sys::string_to_name(value)?;
                Ok(name.into())
            }

            #[inline]
            pub const fn as_u64(&self) -> u64 {
                self.0
            }

            #[inline]
            pub const fn from_u64(value: u64) -> Self {
                #ident(value)
            }
        }

        #[automatically_derived]
        impl #eosio::SecondaryTableKey for #ident {
            #[inline]
            fn end(
                &self,
                code: #eosio::AccountName,
                scope: #eosio::ScopeName,
                table: #eosio::SecondaryTableName
            ) -> i32 {
                u64::from(*self).end(code, scope, table)
            }
            #[inline]
            fn next(&self, iterator: i32) -> (i32, u64) {
                u64::from(*self).next(iterator)
            }
            #[inline]
            fn previous(&self, iterator: i32) -> (i32, u64) {
                u64::from(*self).previous(iterator)
            }
            #[inline]
            fn erase(&self, iterator: i32) {
                u64::from(*self).erase(iterator)
            }
            #[inline]
            fn store(
                &self,
                scope: #eosio::ScopeName,
                table: #eosio::SecondaryTableName,
                payer: #eosio::AccountName,
                id: u64,
            ) -> i32 {
                u64::from(*self).store(scope, table, payer, id)
            }
            #[inline]
            fn modify(&self, iterator: i32, payer: AccountName) {
                u64::from(*self).modify(iterator, payer)
            }
            #[inline]
            fn lower_bound(
                &self,
                code: #eosio::AccountName,
                scope: #eosio::ScopeName,
                table: #eosio::SecondaryTableName,
            ) -> (i32, u64) {
                u64::from(*self).lower_bound(code, scope, table)
            }
            #[inline]
            fn upper_bound(
                &self,
                code: #eosio::AccountName,
                scope: #eosio::ScopeName,
                table: #eosio::SecondaryTableName,
            ) -> (i32, u64) {
                u64::from(*self).upper_bound(code, scope, table)
            }
            #[inline]
            fn find_primary(
                &self,
                code: #eosio::AccountName,
                scope: #eosio::ScopeName,
                table: #eosio::SecondaryTableName,
                primary: u64,
            ) -> i32 {
                u64::from(*self).find_primary(code, scope, table, primary)
            }
            #[inline]
            fn find_secondary(
                &self,
                code: #eosio::AccountName,
                scope: #eosio::ScopeName,
                table: #eosio::SecondaryTableName,
            ) -> (i32, u64) {
                u64::from(*self).find_secondary(code, scope, table)
            }
        }
    };
    expanded.into()
}
