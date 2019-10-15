use eosio::*;

#[eosio::action]
pub fn exec(executer: Ignore<AccountName>, trx: Ignore<Transaction>) {
    require_auth(current_receiver());
    let mut ds = current_data_stream();
    let executer = ds.read::<AccountName>().expect("read");
    require_auth(executer);

    let id: TransactionId = {
        let now = current_time_point().as_i64() as u128;
        let value = u128::from(executer.as_u64()) << 64 | now;
        value.into()
    };
    send_deferred_bytes(&id, executer, &ds, false).expect("write");
}

eosio::abi!(exec);
