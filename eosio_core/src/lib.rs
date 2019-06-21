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
    clippy::pedantic,
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

mod action;
mod asset;
mod block_timestamp;
mod blockchain_parameters;
mod checksum160;
mod checksum256;
mod checksum512;
mod data_stream;
mod extended_asset;
mod extended_symbol;
mod fixed_bytes;
mod ignore;
mod json;
mod names;
mod ops;
mod permission_level;
mod producer_key;
mod producer_schedule;
mod public_key;
mod resources;
mod signature;
mod signed_int;
mod symbol;
mod symbol_code;
mod time_point;
mod time_point_sec;
mod transaction;
mod unsigned_int;

#[doc(inline)]
pub use self::{
    action::*, asset::*, block_timestamp::*, blockchain_parameters::*,
    checksum160::*, checksum256::*, checksum512::*, data_stream::*,
    extended_asset::*, extended_symbol::*, fixed_bytes::*, ignore::*, json::*,
    names::*, ops::*, permission_level::*, producer_key::*,
    producer_schedule::*, public_key::*, resources::*, signature::*,
    signed_int::*, symbol::*, symbol_code::*, time_point::*, time_point_sec::*,
    transaction::*, unsigned_int::*,
};
