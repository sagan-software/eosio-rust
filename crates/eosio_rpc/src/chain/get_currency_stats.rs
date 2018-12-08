use crate::{Builder, Client, Error};
use eosio::AccountName;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GetCurrencyStatsBuilder {
    code: AccountName,
    symbol: String,
}

impl Builder for GetCurrencyStatsBuilder {
    const PATH: &'static str = "/v1/chain/get_currency_stats";
    type Output = GetCurrencyStats;
}

pub type GetCurrencyStats = ::std::collections::HashMap<String, CurrencyStats>;

pub fn get_currency_stats(code: AccountName, symbol: &str) -> GetCurrencyStatsBuilder {
    GetCurrencyStatsBuilder {
        code,
        symbol: symbol.into(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyStats {
    pub supply: String,
    pub max_supply: String,
    pub issuer: AccountName,
}
