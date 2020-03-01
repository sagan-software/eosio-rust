//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/name.hpp#L28-L269>
mod name_type;

use crate::bytes::{NumBytes, Read, Write};
use core::{
    cmp::PartialEq,
    fmt,
    str::{self, FromStr},
};
pub use eosio_numstr::ParseNameError;
use eosio_numstr::{name_from_bytes, name_to_bytes};

/// TODO docs
/// TODO use `NonZeroU64`
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
#[eosio(crate_path = "crate::bytes")]
pub struct Name(u64);

impl Name {
    /// Creates a new name
    #[inline]
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// TODO docs
    #[inline]
    #[must_use]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Name {
    #[inline]
    #[must_use]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<Name> for u64 {
    #[inline]
    #[must_use]
    fn from(i: Name) -> Self {
        i.0
    }
}

impl FromStr for Name {
    type Err = ParseNameError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = name_from_bytes(s.bytes())?;
        Ok(name.into())
    }
}

impl fmt::Display for Name {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = name_to_bytes(self.0);
        let value = str::from_utf8(&bytes)
            .map(|s| s.trim_end_matches('.'))
            .map_err(|_| fmt::Error)?;
        write!(f, "{}", value)
    }
}

impl PartialEq<u64> for Name {
    fn eq(&self, other: &u64) -> bool {
        &self.0 == other
    }
}

#[cfg(test)]
mod tests {
    use super::{FromStr, Name};
    use alloc::string::ToString;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn from_str_to_string(input in "[[1-5][a-z]]{0,12}[a-j]{0,1}") {
            let name = Name::from_str(&input).expect("Failed to parse name from str");
            let string = name.to_string();
            prop_assert_eq!(string, input);
        }
    }
}
