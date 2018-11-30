use eosio::AccountName;
use serde_derive::{Deserialize, Serialize};

const PATH: &str = "/v1/chain/get_currency_stats";

#[derive(Serialize)]
struct Params {
    code: AccountName,
    symbol: String,
}

pub type GetCurrencyStats = ::std::collections::HashMap<String, CurrencyStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyStats {
    supply: String,
    max_supply: String,
    issuer: AccountName,
}

pub fn get_currency_stats<C>(
    node: &str,
    code: C,
    symbol: &str,
) -> impl ::futures::Future<Item = GetCurrencyStats, Error = crate::Error>
where
    C: Into<AccountName>,
{
    crate::http::post(
        node,
        PATH,
        Params {
            code: code.into(),
            symbol: symbol.into(),
        },
    )
}
