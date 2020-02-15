use crate::{NumBytes, Read, Write};
use core::cmp::PartialEq;

macro_rules! checksum_type {
    ($ident:ident, $bytes:literal) => {
        /// TODO docs
        /// TODO Read, Write, `NumBytes` needs a custom implementation based on `fixed_bytes`
        #[derive(
            Debug, Read, Write, NumBytes, Clone, Copy, PartialEq, PartialOrd,
        )]
        #[eosio(crate_path = "crate::bytes")]
        pub struct $ident([u8; $bytes]);

        impl $ident {
            /// TODO docs.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; $bytes] {
                &self.0
            }

            /// TODO docs.
            #[must_use]
            pub const fn to_bytes(&self) -> [u8; $bytes] {
                self.0
            }

            /// TODO docs.
            #[must_use]
            pub fn as_slice(&self) -> &[u8] {
                &self.0
            }
        }

        impl From<[u8; $bytes]> for $ident {
            #[inline]
            #[must_use]
            fn from(value: [u8; $bytes]) -> Self {
                Self(value)
            }
        }

        impl From<$ident> for [u8; $bytes] {
            #[inline]
            #[must_use]
            fn from(value: $ident) -> Self {
                value.0
            }
        }

        impl Default for $ident {
            fn default() -> Self {
                Self([0; $bytes])
            }
        }

        impl AsRef<$ident> for $ident {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl PartialEq<[u8]> for $ident {
            fn eq(&self, other: &[u8]) -> bool {
                &self.0 == other
            }
        }

        impl PartialEq<$ident> for &[u8] {
            fn eq(&self, other: &$ident) -> bool {
                self == &other.0
            }
        }

        impl PartialEq<[u8; $bytes]> for $ident {
            fn eq(&self, other: &[u8; $bytes]) -> bool {
                &self.0 == other
            }
        }

        impl PartialEq<$ident> for [u8; $bytes] {
            fn eq(&self, other: &$ident) -> bool {
                self == &other.0
            }
        }
    };
}

checksum_type!(Checksum160, 20);
checksum_type!(Checksum256, 32);

/// TODO docs
/// TODO Read, Write, `NumBytes` needs a custom implementation based on `fixed_bytes`
#[derive(Read, Write, NumBytes, Clone, Copy)]
#[eosio(crate_path = "crate::bytes")]
pub struct Checksum512([u8; 64]);

impl Checksum512 {
    /// TODO docs.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }

    /// TODO docs.
    #[must_use]
    pub const fn to_bytes(&self) -> [u8; 64] {
        self.0
    }

    /// TODO docs.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 64]> for Checksum512 {
    #[inline]
    #[must_use]
    fn from(value: [u8; 64]) -> Self {
        Self(value)
    }
}

impl From<Checksum512> for [u8; 64] {
    #[inline]
    #[must_use]
    fn from(value: Checksum512) -> Self {
        value.0
    }
}

impl Default for Checksum512 {
    fn default() -> Self {
        Self([0; 64])
    }
}

impl AsRef<Checksum512> for Checksum512 {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}
