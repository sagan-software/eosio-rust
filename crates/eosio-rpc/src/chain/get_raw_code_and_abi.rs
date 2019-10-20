use eosio::AccountName;

const PATH: &str = "/v1/chain/get_raw_code_and_abi";

struct Params {
    account_name: AccountName,
}
