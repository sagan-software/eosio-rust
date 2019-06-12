//! TODO docs
use eosio_bytes::{NumBytes, Read, Write};
use eosio_numstr::{name_from_str, name_to_string};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

pub use eosio_numstr::ParseNameError;

macro_rules! declare_name_types {
    ($($ident:ident)*) => ($(
        /// TODO docs
        #[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash, PartialOrd, Ord, Read, Write, NumBytes, Serialize, Deserialize)]
        #[eosio_bytes_root_path = "::eosio_bytes"]
        pub struct $ident(u64);

        impl $ident {
            /// TODO docs
            #[inline]
            pub const fn as_u64(&self) -> u64 {
                self.0
            }
        }

        impl From<u64> for $ident {
            #[inline]
            fn from(n: u64) -> Self {
                Self(n)
            }
        }

        impl From<$ident> for u64 {
            #[inline]
            fn from(i: $ident) -> Self {
                i.0
            }
        }

        impl FromStr for $ident {
            type Err = ParseNameError;
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let name = name_from_str(s)?;
                Ok(name.into())
            }
        }

        impl TryFrom<&str> for $ident {
            type Error = ParseNameError;
            #[inline]
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::from_str(value)
            }
        }

        impl TryFrom<String> for $ident {
            type Error = ParseNameError;
            #[inline]
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::from_str(value.as_str())
            }
        }

        impl fmt::Display for $ident {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let s = name_to_string(self.0);
                write!(f, "{}", s)
            }
        }

        impl From<$ident> for String {
            #[inline]
            fn from(i: $ident) -> Self {
                i.to_string()
            }
        }

        impl PartialEq<$ident> for String {
            #[inline]
            fn eq(&self, other: &$ident) -> bool {
                self.as_str() == other.to_string().as_str()
            }
        }

        impl PartialEq<String> for $ident {
            #[inline]
            fn eq(&self, other: &String) -> bool {
                self.to_string().as_str() == other.as_str()
            }
        }
    )*)
}

declare_name_types! {
    Name
    AccountName
    PermissionName
    ScopeName
    TableName
    ActionName
}

impl From<ScopeName> for AccountName {
    #[inline]
    fn from(scope: ScopeName) -> Self {
        let value: u64 = scope.into();
        value.into()
    }
}

impl From<AccountName> for ScopeName {
    #[inline]
    fn from(name: AccountName) -> Self {
        let value: u64 = name.into();
        value.into()
    }
}
