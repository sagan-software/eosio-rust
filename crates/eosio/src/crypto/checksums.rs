use crate::{NumBytes, Read, Write};

macro_rules! checksum_type {
    ($ident:ident, $bytes:literal) => {
        /// TODO docs
        /// TODO Read, Write, `NumBytes` needs a custom implementation based on `fixed_bytes`
        #[derive(Read, Write, NumBytes, Clone, Copy)]
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
    };
}

checksum_type!(Checksum160, 20);
checksum_type!(Checksum256, 32);
checksum_type!(Checksum512, 64);
