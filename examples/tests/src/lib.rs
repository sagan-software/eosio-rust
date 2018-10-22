#![feature(proc_macro_hygiene)]

extern crate eosio;

use eosio::*;

#[eosio_action]
fn crypto(data: String) {
    Ripemd160::new(&data).assert(&data);
    Sha1::new(&data).assert(&data);
    Sha256::new(&data).assert(&data);
    Sha512::new(&data).assert(&data);

    for name in AccountName::active_producers().iter() {
        " !!! ".print();
        name.print();
    }

    let alice: AccountName = n!(alice).into();
    alice.permission_last_used(n!(active));
}

eosio_abi!(crypto);
