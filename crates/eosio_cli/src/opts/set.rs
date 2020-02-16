use eosio::{AccountName, ActionName, PermissionName};
use structopt::StructOpt;

/// Set or update blockchain state
#[derive(StructOpt, Debug)]
pub enum Set {
    /// Create or update the code on an account
    Code(SetCode),
    /// Create or update the abi on an account
    Abi(SetAbi),
    /// Create or update the contract on an account
    Contract(SetContract),
    /// set or update blockchain account state
    Account(SetAccount),
    /// set or update blockchain action state
    Action(SetAction),
}

/// Create or update the code on an account
#[derive(StructOpt, Debug)]
pub struct SetCode {
    /// The account to set code for
    pub account: AccountName,
    /// The fullpath containing the contract WASM
    pub code_file: Option<String>,
    /// Remove code on an account
    #[structopt(short, long)]
    pub clear: bool,
    /// Don't check for duplicate
    #[structopt(long)]
    pub suppress_duplicate_check: bool,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Create or update the abi on an account
#[derive(StructOpt, Debug)]
pub struct SetAbi {
    /// The account to set the ABI for
    pub account: AccountName,
    /// The fullpath containing the contract ABI
    pub abi_file: Option<String>,
    /// Remove code on an account
    #[structopt(short, long)]
    pub clear: bool,
    /// Don't check for duplicate
    #[structopt(long)]
    pub suppress_duplicate_check: bool,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Create or update the contract on an account
#[derive(StructOpt, Debug)]
pub struct SetContract {
    /// The account to set the ABI for
    pub account: AccountName,
    /// The path containing the .wasm and .abi
    pub contract_dir: Option<String>,
    /// The file containing the contract WASM relative to contract-dir
    pub wasm_file: Option<String>,
    /// The ABI for the contract relative to contract-dir
    pub abi_file: Option<String>,
    /// The ABI for the contract relative to contract-dir
    #[structopt(short, long)]
    pub abi: Option<String>,
    /// Remove code on an account
    #[structopt(short, long)]
    pub clear: bool,
    /// Don't check for duplicate
    #[structopt(long)]
    pub suppress_duplicate_check: bool,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// set or update blockchain account state
#[derive(StructOpt, Debug)]
pub enum SetAccount {
    /// set parameters dealing with account permissions
    Permission(SetAccountPermission),
}

/// set parameters dealing with account permissions
#[derive(StructOpt, Debug)]
pub struct SetAccountPermission {
    /// The account to set/delete a permission authority for
    pub account: AccountName,
    /// The permission name to set/delete an authority for
    pub permission: PermissionName,
    /// [delete] NULL, [create/update] public key, JSON string or filename
    /// defining the authority, [code] contract name
    pub authority: Option<String>,
    /// [create] The permission name of this parents permission
    #[structopt(default_value = "active")]
    pub parent: PermissionName,
    /// [code] add 'eosio.code' permission to specified permission authority
    #[structopt(long)]
    pub add_code: bool,
    /// [code] remove 'eosio.code' permission from specified permission
    /// authority
    #[structopt(long)]
    pub remove_code: bool,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// set or update blockchain action state
#[derive(StructOpt, Debug)]
pub enum SetAction {
    /// set parameters dealing with account permissions
    Permission(SetActionPermission),
}

/// set parameters dealing with account permissions
#[derive(StructOpt, Debug)]
pub struct SetActionPermission {
    /// The account to set/delete a permission authority for
    pub account: AccountName,
    /// The account that owns the code for the action,
    pub code: AccountName,
    /// the type of action
    pub type_: ActionName,
    /// [delete] NULL, [set/update] The permission name require for executing
    /// the given action
    pub requirement: PermissionName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}
