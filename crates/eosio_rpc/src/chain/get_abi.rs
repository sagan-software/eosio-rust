use eosio::AccountName;
use serde_derive::{Deserialize, Serialize};

const PATH: &str = "/v1/chain/get_abi";

#[derive(Serialize)]
struct Params {
    account_name: AccountName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAbi {
    account_name: AccountName,
    abi: Abi,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Abi {
    version: String,
    types: Vec<Type>,
    structs: Vec<Struct>,
    actions: Vec<Action>,
    tables: Vec<Table>,
    ricardian_clauses: Vec<RicardianClause>,
    error_messages: Vec<ErrorMessage>,
    abi_extensions: Vec<AbiExtension>,
    // TODO variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
    new_type_name: String,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Struct {
    name: String,
    base: String,
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    name: String,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    name: String,
    #[serde(rename = "type")]
    type_: String,
    ricardian_contract: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    name: String,
    index_type: String,
    key_names: Vec<String>,
    key_types: Vec<String>,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RicardianClause {
    id: String,
    body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    error_code: u64,
    error_msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbiExtension {
    #[serde(rename = "type")]
    type_: u16,
    data: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Variant {}

pub fn get_abi<A>(
    node: &str,
    account_name: A,
) -> impl ::futures::Future<Item = GetAbi, Error = crate::Error>
where
    A: Into<AccountName>,
{
    crate::http::post(
        node,
        PATH,
        Params {
            account_name: account_name.into(),
        },
    )
}
