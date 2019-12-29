//! TODO docs
#![no_std]
#![deny(
    clippy::correctness,
    clippy::indexing_slicing,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used,
    clippy::unimplemented,
    clippy::wrong_pub_self_convention,
    clippy::wrong_self_convention
)]
#![warn(
    clippy::complexity,
    clippy::pedantic,
    clippy::nursery,
    clippy::style,
    clippy::perf,
    clippy::cargo,
    clippy::dbg_macro,
    clippy::else_if_without_else,
    clippy::float_cmp_const,
    clippy::mem_forget,
    clippy::use_debug
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    clippy::trivially_copy_pass_by_ref
)]

#[macro_use]
extern crate alloc;

// TODO remove
// extern crate std;

pub use eosio_macros::{action, n, s, table};

mod abi;
pub use self::abi::*;

mod account;
pub use self::account::AccountName;

mod action;
pub use self::action::{
    Action, ActionFn, ActionName, PermissionLevel, PermissionName,
};

mod asset;
pub use self::asset::{Asset, ExtendedAsset};

mod binary_extension;
pub use self::binary_extension::BinaryExtension;

mod block;
pub use self::block::*;

mod blockchain_parameters;
pub use self::blockchain_parameters::*;

mod bytes;
pub use self::bytes::{
    DataStream, NumBytes, Read, ReadError, Write, WriteError,
};

mod crypto;
pub use self::crypto::{
    Checksum160, Checksum256, Checksum512, PrivateKey, PublicKey, Signature,
};

#[cfg(feature = "serde")]
mod json;
#[cfg(feature = "serde")]
pub use self::json::*;

#[macro_use]
mod name;
pub use self::name::{Name, NAME_LEN_MAX, NAME_UTF8_CHARS};

mod ops;
pub use self::ops::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub,
};

mod producer_schedule;
pub use self::producer_schedule::*;

mod resources;
pub use self::resources::{CpuWeight, NetWeight, RamBytes};

mod symbol;
pub use self::symbol::{
    ExtendedSymbol, Symbol, SymbolCode, SYMBOL_LEN_MAX, SYMBOL_UTF8_CHARS,
};

mod table;
pub use self::table::{
    PrimaryTableIndex, ScopeName, SecondaryKey, SecondaryKeys,
    SecondaryTableIndex, SecondaryTableName, Table, TableName,
};

mod time;
pub use self::time::{BlockTimestamp, TimePoint, TimePointSec};

mod transaction;
pub use self::transaction::{
    Transaction, TransactionExtension, TransactionHeader, TransactionId,
};

mod varint;
pub use self::varint::{SignedInt, UnsignedInt};
