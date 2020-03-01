//! This crate provides functions for converting EOSIO names and
//! symbols to and from string representations.
//!
//! Creating an EOSIO name:
//!
//! ```
//! use eosio_numstr::name_from_bytes;
//! let name = name_from_bytes("eosio".bytes()).unwrap();
//! assert_eq!(name, 6138663577826885632);
//! ```
//!
//! Creating an EOSIO symbol:
//!
//! ```
//! use eosio_numstr::symbol_from_bytes;
//! let symbol = symbol_from_bytes(4, "EOS".bytes()).unwrap();
//! assert_eq!(symbol, 1397703940);
//! ```
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
    clippy::dbg_macro,
    clippy::else_if_without_else,
    clippy::float_cmp_const,
    clippy::mem_forget,
    clippy::use_debug
)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(
    test,
    allow(clippy::option_unwrap_used, clippy::result_unwrap_used)
)]

#[cfg(test)]
#[macro_use]
extern crate std;

mod name;
mod symbol;
mod symbol_code;

pub use name::{
    name_from_bytes, name_to_bytes, ParseNameError, NAME_CHARS, NAME_MAX_LEN,
};
pub use symbol::{
    symbol_from_bytes, symbol_from_code, symbol_to_code, symbol_to_precision,
    ParseSymbolError,
};
pub use symbol_code::{
    symbol_code_from_bytes, symbol_code_to_bytes, ParseSymbolCodeError,
    SYMBOL_CODE_CHARS, SYMBOL_CODE_MAX_LEN,
};
