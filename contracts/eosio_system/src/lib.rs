use eosio::*;
use lazy_static::lazy_static;

pub const ACTIVE_PERMISSION: PermissionName = PermissionName::new(n!("active"));
pub const TOKEN_ACCOUNT: AccountName = AccountName::new(n!("eosio.token"));
pub const RAM_ACCOUNT: AccountName = AccountName::new(n!("eosio.ram"));
pub const RAMFEE_ACCOUNT: AccountName = AccountName::new(n!("eosio.ramfee"));
pub const STAKE_ACCOUNT: AccountName = AccountName::new(n!("eosio.stake"));
pub const BPAY_ACCOUNT: AccountName = AccountName::new(n!("eosio.bpay"));
pub const VPAY_ACCOUNT: AccountName = AccountName::new(n!("eosio.vpay"));
pub const NAMES_ACCOUNT: AccountName = AccountName::new(n!("eosio.names"));
pub const SAVING_ACCOUNT: AccountName = AccountName::new(n!("eosio.saving"));
pub const REX_ACCOUNT: AccountName = AccountName::new(n!("eosio.rex"));
pub const NULL_ACCOUNT: AccountName = AccountName::new(n!("eosio.null"));
pub const REX_SYMBOL: Symbol = Symbol::new(s!(4, "REX"));

lazy_static! {
    pub static ref SELF: AccountName = eosio_cdt::current_receiver();
}

mod core;
pub use self::core::*;

mod delegate_bandwidth;
pub use self::delegate_bandwidth::*;

mod exchange_state;
pub use self::exchange_state::*;

mod name_bidding;
pub use self::name_bidding::*;

mod native;
pub use self::native::*;

mod producer_pay;
pub use self::producer_pay::*;

mod rex;
pub use self::rex::*;

mod voting;
pub use self::voting::*;

eosio::abi!(
    setram,
    setramrate,
    setparams,
    setpriv,
    setalimits,
    setacctram,
    setacctnet,
    activate,
    rmvproducer,
    updtrevision,
    setinflation,
    init
);
