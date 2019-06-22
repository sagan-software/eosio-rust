use eosio::{AccountName, ActionName};
use serde::Serialize;

const PATH: &str = "/v1/chain/abi_json_to_bin";

struct Params<Args>
where
    Args: Serialize,
{
    code: AccountName,
    action: ActionName,
    args: Args,
}
