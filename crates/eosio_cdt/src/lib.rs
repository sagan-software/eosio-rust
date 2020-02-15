//! TODO docs
#![no_std]
#![recursion_limit = "128"]
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
    clippy::missing_inline_in_public_items,
    clippy::use_debug
)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate alloc;

mod account;
pub use self::account::*;

mod action;
pub use self::action::*;

mod check;
pub use self::check::*;

mod crypto;
pub use self::crypto::*;

mod permissions;
pub use self::permissions::*;

#[macro_use]
mod print;
pub use self::print::*;

mod privileged;
pub use self::privileged::*;

mod singleton_index;
pub use self::singleton_index::*;

mod table;
pub use self::table::*;

mod table_primary;
pub use self::table_primary::*;

mod table_secondary;
pub use self::table_secondary::*;

mod time;
pub use self::time::*;

pub use eosio_cdt_sys as sys;

// pub use self::{
//     account::*, action::*, check::*, crypto::*, permissions::*, print::*,
//     privileged::*, singleton_index::*, table::*, table_primary::*,
//     table_secondary::*, time::*,
// };
