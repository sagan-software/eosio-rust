use eosio::*;
use eosio_cdt::*;
use std::marker::PhantomData;

#[eosio::action]
pub fn exec(
    executer: PhantomData<AccountName>,
    trx: PhantomData<Transaction<Vec<u8>>>,
) {
    require_auth(current_receiver());
    let mut ds = current_data_stream();
    let executer = ds.read::<AccountName>().expect("read");
    require_auth(executer);

    let id: TransactionId = {
        let now = current_time_point().as_micros() as u128;
        let value = u128::from(executer.as_u64()) << 64 | now;
        value.into()
    };

    let bytes = ds.as_remaining_bytes().unwrap();
    send_deferred_bytes(id, executer, bytes, false);
}

eosio::abi!(exec);
