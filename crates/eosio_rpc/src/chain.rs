use eosio::AccountName;

pub type ChainId = String;

pub type BlockId = String;

pub type BlockNum = u32;

pub type ServerVersion = String;

pub type BlockTimestamp = String;

#[derive(::serde::Deserialize, ::serde::Serialize, Debug)]
pub struct Info {
    pub server_version: ServerVersion,
    pub server_version_string: String,
    pub chain_id: ChainId,
    pub head_block_num: BlockNum,
    pub head_block_id: BlockId,
    pub head_block_time: BlockTimestamp,
    pub head_block_producer: AccountName,
    pub last_irreversible_block_num: BlockNum,
    pub last_irreversible_block_id: BlockId,
    pub virtual_block_cpu_limit: u32,
    pub virtual_block_net_limit: u32,
    pub block_cpu_limit: u32,
    pub block_net_limit: u32,
}

pub fn get_info(node: &str) {}

pub fn get_block() {}

pub fn get_block_header_state() {}

pub fn get_account() {}

pub fn get_abi() {}

pub fn get_code() {}

pub fn get_raw_code_and_abi() {}

#[derive(::serde::Serialize, ::serde::Deserialize, Debug, Clone, Default)]
pub struct TableRows<Row> {
    pub rows: Vec<Row>,
    pub more: bool,
}

pub fn get_table_rows() {}

pub fn get_currency_balance() {}

pub fn abi_json_to_bin() {}

pub fn abi_bin_to_json() {}

pub fn get_required_keys() {}

pub fn get_currency_stats() {}

pub fn get_producers() {}

pub fn push_block() {}

pub fn push_transaction() {}

pub fn push_transactions() {}
