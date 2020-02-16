use eosio::{AccountName, ActionName};
use structopt::StructOpt;

/// Pack and unpack transactions
#[derive(StructOpt, Debug)]
pub enum Convert {
    /// From plain signed json to packed form
    PackTransaction(ConvertPackTransaction),
    /// From packed to plain signed json form
    UnpackTransaction(ConvertUnpackTransaction),
    /// From json action data to packed form
    PackActionData(ConvertPackActionData),
    /// From packed to json action data form
    UnpackActionData(ConvertUnpackActionData),
}

/// From plain signed json to packed form
#[derive(StructOpt, Debug)]
pub struct ConvertPackTransaction {
    /// The plain signed json
    pub transaction: String,
    /// Pack all action data within transaction, needs interaction with nodeos
    #[structopt(long)]
    pub pack_action_data: bool,
}

/// From packed to plain signed json form
#[derive(StructOpt, Debug)]
pub struct ConvertUnpackTransaction {
    /// The packed transaction json (string containing packed_trx and
    /// optionally compression fields)
    pub transaction: String,
    /// Unpack all action data within transaction, needs interaction with
    /// nodeos
    #[structopt(long)]
    pub unpack_action_data: bool,
}

/// From json action data to packed form
#[derive(StructOpt, Debug)]
pub struct ConvertPackActionData {
    /// The name of the account that hosts the contract
    pub account: AccountName,
    /// The name of the function that's called by this action
    pub name: ActionName,
    /// The action data expressed as json
    pub unpacked_action_data: String,
}

/// From json action data to packed form
#[derive(StructOpt, Debug)]
pub struct ConvertUnpackActionData {
    /// The name of the account that hosts the contract
    pub account: AccountName,
    /// The name of the function that's called by this action
    pub name: ActionName,
    /// The action data expressed as packed hex string
    pub packed_action_data: String,
}
