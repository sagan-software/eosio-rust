use eosio::{AccountName, ScopeName, TableName};
use serde::{Deserialize, Serialize};

const PATH: &str = "/v1/chain/get_table_rows";

#[derive(Serialize)]
struct Params {
    scope: ScopeName,
    code: AccountName,
    table: TableName,
    json: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    lower_bound: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upper_bound: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetTableRows<Row> {
    pub rows: Vec<Row>,
    pub more: bool,
}

pub fn get_table_rows<Row>(
    node: &str,
    scope: ScopeName,
    code: AccountName,
    table: TableName,
    lower_bound: Option<u64>,
    upper_bound: Option<u64>,
    limit: Option<u32>,
) -> impl ::futures::Future<Item = GetTableRows<Row>, Error = crate::Error>
where
    Row: for<'a> Deserialize<'a>,
{
    crate::http::post(
        node,
        PATH,
        Params {
            scope,
            code,
            table,
            json: true,
            lower_bound,
            upper_bound,
            limit,
        },
    )
}
