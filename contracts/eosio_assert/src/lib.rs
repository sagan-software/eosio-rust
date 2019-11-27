use eosio::*;

pub struct StoredChainParams {
    pub chain_id: Checksum256,
    pub chain_name: String,
    pub icon: Checksum256,
    pub hash: Checksum256,
    pub next_unique_id: u64,
}

pub struct ContractAction {
    pub contract: AccountName,
    pub action: ActionName,
}

pub struct StoredManifest {
    pub unique_id: u64,
    pub id: Checksum256,
    pub account: AccountName,
    pub domain: String,
    pub appmeta: String,
    pub whitelist: Vec<ContractAction>,
}

pub struct AbiHash {
    pub owner: AccountName,
    pub hash: Checksum256,
}

#[eosio::action]
pub fn setchain(
    _chain_id: Ignore<Checksum256>,
    _chain_name: Ignore<String>,
    _icon: Ignore<Checksum256>,
) {
    require_auth(n!("eosio"));
}

// #[eosio::action("add.manifest")]
pub fn add_manifest(
    account: Ignore<AccountName>,
    domain: Ignore<String>,
    appmeta: Ignore<String>,
    whitelist: Ignore<Vec<ContractAction>>,
) {
}

// #[eosio::action("del.manifest")]
pub fn del_manifest(id: Checksum256) {}

// #[eosio::action]
pub fn require(
    chain_params_hash: Checksum256,
    manifest_id: Checksum256,
    actions: Vec<ContractAction>,
    abi_hashes: Vec<Checksum256>,
) {
}
