use crate::{AccountName, NumBytes, Read, UnsignedInt, Write};

macro_rules! key_type {
    ($ident:ident, $bytes:literal) => {
        /// EOSIO Public Key
        /// <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/crypto.hpp#L22-L48>
        #[derive(Read, Write, NumBytes, Clone)]
        #[__eosio_path = "crate::bytes"]
        pub struct $ident {
            /// Type of the public key, could be either K1 or R1
            pub type_: UnsignedInt,
            /// Bytes of the public key
            pub data: [u8; $bytes],
        }

        impl $ident {
            /// TODO docs.
            pub fn as_bytes(&self) -> &[u8] {
                &self.data
            }

            /// TODO docs.
            pub const fn to_bytes(&self) -> [u8; $bytes] {
                self.data
            }
        }

        impl Default for $ident {
            fn default() -> Self {
                Self {
                    type_: UnsignedInt::default(),
                    data: [0_u8; $bytes],
                }
            }
        }

        impl PartialEq for $ident {
            fn eq(&self, other: &Self) -> bool {
                self.type_ == other.type_ && self.as_bytes() == other.as_bytes()
            }
        }

        impl std::fmt::Debug for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.type_, f)?;
                std::fmt::Debug::fmt(self.as_bytes(), f)
            }
        }
    };
}

key_type!(PublicKey, 34);
key_type!(Signature, 66);

/// TODO docs
#[derive(Read, Write, NumBytes, Clone)]
#[__eosio_path = "crate::bytes"]
pub struct PrivateKey(String);

/// Maps producer with its signing key, used for producer schedule
/// <https://github.com/EOSIO/eosio.cdt/blob/796ff8bee9a0fc864f665a0a4d018e0ff18ac383/libraries/eosiolib/contracts/eosio/producer_schedule.hpp#L15-L45>
#[derive(Read, Write, NumBytes, Clone, Default)]
#[__eosio_path = "crate::bytes"]
pub struct ProducerKey {
    /// Name of the producer
    pub producer_name: AccountName,
    /// Block signing key used by this producer
    pub block_signing_key: PublicKey,
}
