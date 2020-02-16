use alloc::{string::String, vec::Vec};

#[derive(Debug, PartialEq)]
pub struct Abi {
    pub version: String,
    pub types: Vec<AbiType>,
    pub structs: Vec<AbiStruct>,
    pub actions: Vec<AbiAction>,
    pub tables: Vec<AbiTable>,
    pub ricardian_clauses: Vec<AbiRicardianClause>,
    pub error_messages: Vec<AbiErrorMessage>,
    pub abi_extensions: Vec<AbiExtension>,
    // TODO variants: Vec<Variant>,
}

#[derive(Debug, PartialEq)]
pub struct AbiType {
    pub new_type_name: String,
    pub type_: String,
}

#[derive(Debug, PartialEq)]
pub struct AbiStruct {
    pub name: String,
    pub base: String,
    pub fields: Vec<AbiField>,
}

#[derive(Debug, PartialEq)]
pub struct AbiField {
    pub name: String,
    pub type_: String,
}

#[derive(Debug, PartialEq)]
pub struct AbiAction {
    pub name: String,
    pub type_: String,
    pub ricardian_contract: String,
}

#[derive(Debug, PartialEq)]
pub struct AbiTable {
    pub name: String,
    pub index_type: String,
    pub key_names: Vec<String>,
    pub key_types: Vec<String>,
    pub type_: String,
}

#[derive(Debug, PartialEq)]
pub struct AbiRicardianClause {
    pub id: String,
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub struct AbiErrorMessage {
    pub error_code: u64,
    pub error_msg: String,
}

#[derive(Debug, PartialEq)]
pub struct AbiExtension {
    pub type_: u16,
    pub data: String,
}
