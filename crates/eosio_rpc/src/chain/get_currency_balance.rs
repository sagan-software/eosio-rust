use eosio::AccountName;
use serde_derive::Serialize;

const PATH: &str = "/v1/chain/get_currency_balance";

#[derive(Serialize)]
struct Params {
    code: AccountName,
    account: AccountName,
    symbol: String,
}

pub type GetCurrencyBalance = Vec<String>;

pub fn get_currency_balance<C, A>(
    node: &str,
    code: C,
    account: A,
    symbol: &str,
) -> impl ::futures::Future<Item = GetCurrencyBalance, Error = crate::Error>
where
    C: Into<AccountName>,
    A: Into<AccountName>,
{
    crate::http::post(
        node,
        PATH,
        Params {
            code: code.into(),
            account: account.into(),
            symbol: symbol.into(),
        },
    )
}
