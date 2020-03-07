#![no_std]
use core::marker::PhantomData;

pub fn noop(_: PhantomData<()>) {}

eosio::abi! {
    noop
}
