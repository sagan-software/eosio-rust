use core::marker::PhantomData;
use eosio::Name;

#[eosio::action]
fn noop(_data: PhantomData<Name>) {}

eosio::abi! {
    noop
}
