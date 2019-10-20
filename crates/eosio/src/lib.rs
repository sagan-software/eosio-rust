//! TODO docs
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
    // clippy::pedantic,
    clippy::nursery,
    clippy::style,
    clippy::perf,
    clippy::cargo,
    clippy::dbg_macro,
    clippy::else_if_without_else,
    clippy::float_cmp_const,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::use_debug
)]
#![allow(
    clippy::missing_inline_in_public_items,
    clippy::module_name_repetitions,
    clippy::trivially_copy_pass_by_ref
)]

pub use eosio_macros::{action, n, s, table};

mod abi;
pub use self::abi::*;

mod account;
pub use self::account::*;

mod action;
pub use self::action::{
    Action, ActionFn, ActionName, PermissionLevel, PermissionName, ToAction,
};

mod asset;
pub use self::asset::*;

mod block;
pub use self::block::*;

mod blockchain_parameters;
pub use self::blockchain_parameters::*;

mod bytes;
pub use self::bytes::*;

mod crypto;
pub use self::crypto::*;

mod ignore;
pub use self::ignore::*;

mod json;
pub use self::json::*;

#[macro_use]
mod name;
pub use self::name::*;

mod ops;
pub use self::ops::*;

mod producer_schedule;
pub use self::producer_schedule::*;

mod resources;
pub use self::resources::*;

mod symbol;
pub use self::symbol::*;

mod table;
pub use self::table::*;

mod time;
pub use self::time::*;

mod transaction;
pub use self::transaction::*;

mod varint;
pub use self::varint::*;
