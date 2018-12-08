use crate::{Client, Error};
use eosio::{AccountName, ScopeName, TableName};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Serialize)]
pub struct GetTableRowsBuilder<Row> {
    #[serde(skip)]
    _data: PhantomData<Row>,
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

impl<Row> crate::builder::Builder for GetTableRowsBuilder<Row>
where
    Row: for<'a> Deserialize<'a> + 'static,
{
    const PATH: &'static str = "/v1/chain/get_table_rows";
    type Output = GetTableRows<Row>;
}

impl<Row> GetTableRowsBuilder<Row> {
    pub fn json(&mut self, value: bool) -> &mut Self {
        self.json = value;
        self
    }

    pub fn lower_bound<V: Into<u64>>(&mut self, value: V) -> &mut Self {
        self.lower_bound = Some(value.into());
        self
    }

    pub fn no_lower_bound(&mut self) -> &mut Self {
        self.lower_bound = None;
        self
    }

    pub fn upper_bound<V: Into<u64>>(&mut self, value: V) -> &mut Self {
        self.upper_bound = Some(value.into());
        self
    }

    pub fn no_upper_bound(&mut self) -> &mut Self {
        self.upper_bound = None;
        self
    }

    pub fn limit(&mut self, value: u32) -> &mut Self {
        self.limit = Some(value);
        self
    }

    pub fn no_limit(&mut self) -> &mut Self {
        self.limit = None;
        self
    }
}

pub fn get_table_rows<Row>(
    code: AccountName,
    scope: ScopeName,
    table: TableName,
) -> GetTableRowsBuilder<Row>
where
    Row: for<'a> Deserialize<'a> + 'static,
{
    GetTableRowsBuilder {
        _data: PhantomData,
        code,
        scope,
        table,
        json: true,
        lower_bound: None,
        upper_bound: None,
        limit: None,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetTableRows<Row> {
    pub rows: Vec<Row>,
    pub more: bool,
}
