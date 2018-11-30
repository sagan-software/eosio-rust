use eosio::AccountName;

const PATH: &str = "/v1/chain/get_code";

struct Params {
    account_name: AccountName,
}
