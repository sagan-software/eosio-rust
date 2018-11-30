use eosio::{AccountName, ActionName};

const PATH: &str = "/v1/chain/abi_bin_to_json";

struct Params {
    code: AccountName,
    action: ActionName,
    binargs: String,
}
