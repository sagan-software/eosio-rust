use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Abi {
    pub version: String,
    pub types: Vec<Type>,
    pub structs: Vec<Struct>,
    pub actions: Vec<Action>,
    pub tables: Vec<Table>,
    pub ricardian_clauses: Vec<RicardianClause>,
    pub error_messages: Vec<ErrorMessage>,
    pub abi_extensions: Vec<AbiExtension>,
    // TODO variants: Vec<Variant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
    pub new_type_name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Struct {
    pub name: String,
    pub base: String,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub ricardian_contract: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub name: String,
    pub index_type: String,
    pub key_names: Vec<String>,
    pub key_types: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RicardianClause {
    pub id: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub error_code: u64,
    pub error_msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbiExtension {
    #[serde(rename = "type")]
    pub type_: u16,
    pub data: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Variant {}
