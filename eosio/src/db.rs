use eosio_types::*;

pub fn find_i64(code: AccountName, scope: AccountName, table: TableName, id: u64) -> i32 {
    unsafe { ::eosio_sys::db::db_find_i64(code.as_u64(), scope.as_u64(), table.as_u64(), id) }
}
