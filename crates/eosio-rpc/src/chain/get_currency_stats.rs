use eosio::AccountName;
use serde::{Deserialize, Serialize};

crate::builder!(
    "/v1/chain/get_currency_stats",
    GetCurrencyStatsParams,
    GetCurrencyStats
);

#[derive(Serialize, Clone)]
pub struct GetCurrencyStatsParams {
    code: AccountName,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
}

pub type GetCurrencyStats = ::std::collections::HashMap<String, CurrencyStats>;

pub fn get_currency_stats<C: Into<AccountName>, S: ToString>(
    code: C,
    symbol: Option<S>,
) -> GetCurrencyStatsParams {
    GetCurrencyStatsParams {
        code: code.into(),
        symbol: symbol.map(|s| s.to_string()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyStats {
    pub supply: String,
    pub max_supply: String,
    pub issuer: AccountName,
}
