//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/name.hpp#L28-L269>
use crate::bytes::{NumBytes, Read, Write};
use eosio_numstr::{name_from_str, name_to_string};
pub use eosio_numstr::{ParseNameError, NAME_LEN_MAX, NAME_UTF8_CHARS};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// TODO docs
struct NameVisitor<
    T: FromStr<Err = ParseNameError> + From<u64> + std::fmt::Display,
>(std::marker::PhantomData<T>);

impl<'de, T> serde::de::Visitor<'de> for NameVisitor<T>
where
    T: FromStr<Err = ParseNameError> + From<u64> + std::fmt::Display,
{
    type Value = T;

    #[inline]
    fn expecting(
        &self,
        formatter: &mut ::std::fmt::Formatter,
    ) -> ::std::fmt::Result {
        formatter.write_str("an EOSIO name string or number")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        value.parse::<T>().map_err(serde::de::Error::custom)
    }

    #[inline]
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Hash,
    PartialOrd,
    Ord,
    Read,
    Write,
    NumBytes,
)]
#[__eosio_path = "crate::bytes"]
pub struct Name(u64);

impl Name {
    /// Creates a new name
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// TODO docs
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Name {
    #[inline]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<Name> for u64 {
    #[inline]
    fn from(i: Name) -> Self {
        i.0
    }
}

impl FromStr for Name {
    type Err = ParseNameError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = name_from_str(s)?;
        Ok(name.into())
    }
}

impl TryFrom<&str> for Name {
    type Error = ParseNameError;
    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for Name {
    type Error = ParseNameError;
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl fmt::Display for Name {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = name_to_string(self.0);
        write!(f, "{}", s)
    }
}

impl From<Name> for String {
    #[inline]
    fn from(i: Name) -> Self {
        i.to_string()
    }
}

impl PartialEq<Name> for String {
    #[inline]
    fn eq(&self, other: &Name) -> bool {
        self.as_str() == other.to_string().as_str()
    }
}

impl PartialEq<String> for Name {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.to_string().as_str() == other.as_str()
    }
}

impl<'de> serde::Deserialize<'de> for Name {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_any(NameVisitor(std::marker::PhantomData::<Self>))
    }
}

impl serde::Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[macro_export]
macro_rules! name_type {
    ($ident:ident) => {
        #[derive(
            Debug,
            PartialEq,
            Eq,
            Clone,
            Copy,
            Default,
            Hash,
            PartialOrd,
            Ord,
            serde::Serialize,
            serde::Deserialize,
            crate::bytes::Read,
            crate::bytes::Write,
            crate::bytes::NumBytes,
        )]
        #[__eosio_path = "crate::bytes"]
        pub struct $ident($crate::name::Name);

        impl $ident {
            pub const fn new(value: u64) -> Self {
                Self($crate::name::Name::new(value))
            }

            pub const fn as_u64(&self) -> u64 {
                self.0.as_u64()
            }

            pub const fn as_name(&self) -> $crate::name::Name {
                self.0
            }
        }

        impl std::ops::Deref for $ident {
            type Target = $crate::name::Name;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::convert::AsRef<$crate::name::Name> for $ident {
            fn as_ref(&self) -> &$crate::name::Name {
                &self.0
            }
        }

        impl std::convert::AsRef<$ident> for $ident {
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl From<u64> for $ident {
            fn from(value: u64) -> Self {
                Self::new(value)
            }
        }

        impl From<$ident> for u64 {
            fn from(value: $ident) -> Self {
                value.as_u64()
            }
        }

        impl From<$crate::name::Name> for $ident {
            fn from(value: $crate::name::Name) -> Self {
                Self(value)
            }
        }

        impl From<$ident> for $crate::name::Name {
            fn from(value: $ident) -> Self {
                value.as_name()
            }
        }
    };
}
