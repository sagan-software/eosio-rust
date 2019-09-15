//! TODO docs
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
    clippy::cargo,
    clippy::dbg_macro,
    clippy::else_if_without_else,
    clippy::float_cmp_const,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::use_debug
)]
#![allow(clippy::module_name_repetitions)]

mod account;
mod action;
mod check;
mod crypto;
mod permissions;
mod print;
mod privileged;
mod singleton_index;
mod table;
mod table_primary;
mod table_secondary;
mod time;

pub use self::{
    account::*, action::*, check::*, crypto::*, permissions::*, print::*,
    privileged::*, singleton_index::*, table::*, table_primary::*,
    table_secondary::*, time::*,
};
