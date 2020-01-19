//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/contracts/eosio/privileged.hpp#L40-L160>
use crate::bytes::{NumBytes, Read, Write};
use alloc::string::String;

/// Tunable blockchain configuration that can be changed via consensus
#[derive(
    Read, Write, NumBytes, Clone, Default, Debug, PartialEq, PartialOrd,
)]
#[eosio(crate_path = "crate::bytes")]
pub struct ChainId(String);

/// Tunable blockchain configuration that can be changed via consensus
#[derive(
    Read, Write, NumBytes, Clone, Default, Debug, PartialEq, PartialOrd,
)]
#[eosio(crate_path = "crate::bytes")]
pub struct BlockchainParameters {
    /// The maximum net usage in instructions for a block
    pub max_block_net_usage: u64,
    /// The target percent (1% == 100, 100% == 10,000) of maximum net usage;
    /// exceeding this triggers congestion handling
    pub target_block_net_usage_pct: u32,
    /// The maximum objectively measured net usage that the chain will
    /// allow regardless of account limits
    pub max_transaction_net_usage: u32,
    /// The base amount of net usage billed for a transaction to cover
    /// incidentals
    pub base_per_transaction_net_usage: u32,
    /// The amount of net usage leeway available whilst executing a
    /// transaction (still checks against new limits wihout leeway at the end
    /// of the transaction)
    pub net_usage_leeway: u32,
    /// The numerator for the discount on net usage of context-free data
    pub context_free_discount_net_usage_num: u32,
    /// The denominator for the discount on net usage of context-free data
    pub context_free_discount_net_usage_den: u32,
    /// The maximum billable cpu usage (in microseconds) for a block
    pub max_block_cpu_usage: u32,
    /// The target percent (1% == 100, 100% == 10,000) of maximum cpu usage;
    /// exceeding this triggers congestion handling
    pub target_block_cpu_usage_pct: u32,
    /// The maximum billable cpu usage (in microseconds) that the chain will
    /// allow regardless of account limits
    pub max_transaction_cpu_usage: u32,
    /// The minimum billable cpu usage (in microseconds) that the chain
    /// requires
    pub min_transaction_cpu_usage: u32,
    /// Maximum lifetime of a transaction
    pub max_transaction_lifetime: u32,
    /// The number of seconds after the time a deferred transaction can first
    /// execute until it expires
    pub deferred_trx_expiration_window: u32,
    /// The maximum number of seconds that can be imposed as a delay
    /// requirement by authorization checks
    pub max_transaction_delay: u32,
    /// Maximum size of inline action
    pub max_inline_action_size: u32,
    /// Maximum depth of inline action
    pub max_inline_action_depth: u16,
    /// Maximum authority depth
    pub max_authority_depth: u16,
}

impl AsRef<BlockchainParameters> for BlockchainParameters {
    fn as_ref(&self) -> &Self {
        self
    }
}
