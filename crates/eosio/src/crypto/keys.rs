use crate::{NumBytes, Read, UnsignedInt, Write};
use alloc::string::String;
use core::fmt;

macro_rules! key_type {
    ($ident:ident, $bytes:literal) => {
        /// TODO depreciate, newer signature types cannot be represented as a
        /// fixed size structure EOSIO Public Key
        /// <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/crypto.hpp#L22-L48>
        #[derive(Read, Write, NumBytes, Clone)]
        #[eosio(crate_path = "crate::bytes")]
        pub struct $ident {
            /// Type of the public key, could be either K1 or R1
            pub type_: UnsignedInt,
            /// Bytes of the public key
            pub data: [u8; $bytes],
        }

        impl $ident {
            /// TODO docs.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; $bytes] {
                &self.data
            }

            /// TODO docs.
            #[must_use]
            pub const fn to_bytes(&self) -> [u8; $bytes] {
                self.data
            }

            /// TODO docs.
            #[must_use]
            pub fn as_slice(&self) -> &[u8] {
                &self.data
            }
        }

        impl Default for $ident {
            #[must_use]
            fn default() -> Self {
                Self {
                    type_: UnsignedInt::default(),
                    data: [0_u8; $bytes],
                }
            }
        }

        impl PartialEq for $ident {
            #[must_use]
            fn eq(&self, other: &Self) -> bool {
                self.type_ == other.type_ && self.as_slice() == other.as_slice()
            }
        }

        impl fmt::Debug for $ident {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(&self.type_, f)?;
                fmt::Debug::fmt(self.as_slice(), f)
            }
        }
    };
}

key_type!(PublicKey, 34);
key_type!(Signature, 66);

/// TODO docs
#[derive(Read, Write, NumBytes, Clone)]
#[eosio(crate_path = "crate::bytes")]
pub struct PrivateKey(String);
