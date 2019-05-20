use eosio::{AccountName, ScopeName, TableName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct GetTableRowsParams {
    pub scope: ScopeName,
    pub code: AccountName,
    pub table: TableName,
    pub json: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_bound: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_bound: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetTableRowsParams {
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

    // TODO re-enable
    // pub fn fetch<Row>(
    //     self,
    //     client: &crate::Client,
    // ) -> impl futures::future::Future<Item = GetTableRows<Row>, Error = crate::Error>
    // where
    //     Row: for<'a> Deserialize<'a> + 'static,
    // {
    //     client.fetch::<GetTableRows<Row>, GetTableRowsParams>("/v1/chain/get_table_rows", self)
    // }
}

pub fn get_table_rows<
    C: Into<AccountName>,
    S: Into<ScopeName>,
    T: Into<TableName>,
>(
    code: C,
    scope: S,
    table: T,
) -> GetTableRowsParams {
    GetTableRowsParams {
        code: code.into(),
        scope: scope.into(),
        table: table.into(),
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
