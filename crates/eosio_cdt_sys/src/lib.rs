//! This crate provides low-level FFI bindings for EOSIO smart contract
//! development. Bindings are automatically generated with [`bindgen`]
//! from header files in the [`EOSIO/eosio.cdt`] repository.
//!
//! For more idiomatic Rust wrappers please see the [`eosio`] and
//! [`eosio_cdt`] crates.
//!
//! [`bindgen`]: https://github.com/rust-lang/rust-bindgen
//! [`EOSIO/eosio.cdt`]: https://github.com/EOSIO/eosio.cdt
//! [`eosio`]: https://crates.io/crates/eosio
//! [`eosio_cdt`]: https://crates.io/crates/eosio_cdt
#![no_std]
#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes,
    dead_code,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::missing_const_for_fn,
    clippy::op_ref,
    clippy::use_self,
    clippy::unseparated_literal_suffix
)]

// #[cfg(target_arch = "wasm32")]
mod bindings;

// #[cfg(target_arch = "wasm32")]
pub use self::bindings::*;

// #[cfg(any(not(target_arch = "wasm32"), feature = "mock"))]
// mod mock_bindings;

// #[cfg(any(not(target_arch = "wasm32"), feature = "mock"))]
// pub use self::mock_bindings::*;

pub use core::ffi::c_void;
pub type c_char = u8;
pub type c_int = i32;
