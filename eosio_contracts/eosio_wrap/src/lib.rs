use eosio::*;

#[eosio::action]
pub fn exec(executer: AccountName, trx: Transaction) {
    require_auth(current_receiver());
    require_auth(executer);
    let id: DeferredTransactionId = {
        let now = current_time().as_i64() as u128;
        let value = u128::from(executer.as_u64()) << 64 | now;
        value.into()
    };
    send_deferred(&id, executer, &trx, false).expect("write");
}

eosio::abi!(exec);
