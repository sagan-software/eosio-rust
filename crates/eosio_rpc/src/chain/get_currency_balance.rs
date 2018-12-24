use crate::Builder;
use eosio::AccountName;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct GetCurrencyBalanceBuilder {
    code: AccountName,
    account: AccountName,
    symbol: String,
}

impl Builder for GetCurrencyBalanceBuilder {
    const PATH: &'static str = "/v1/chain/get_currency_balance";
    type Output = GetCurrencyBalance;
}

pub fn get_currency_balance(
    code: AccountName,
    account: AccountName,
    symbol: &str,
) -> GetCurrencyBalanceBuilder {
    GetCurrencyBalanceBuilder {
        code,
        account,
        symbol: symbol.to_string(),
    }
}

pub type GetCurrencyBalance = Vec<String>;
