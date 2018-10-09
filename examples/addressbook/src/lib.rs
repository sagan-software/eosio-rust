#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::*;

#[eosio_action]
fn add(
    account: AccountName,
    first_name: String,
    last_name: String,
    street: String,
    city: String,
    state: String,
    zip: u32,
) {
    require_auth(account);

    let code = current_receiver();
    let addresses = Address::table(code, code);

    let itr = addresses.find(account);
    eosio_assert!(itr.is_none(), "Address for account already exists");

    let address = Address {
        account,
        first_name,
        last_name,
        street,
        city,
        state,
        zip,
        liked: 0,
    };
    addresses.emplace(account, &address).assert("write");
}

#[eosio_action]
fn update(
    account: AccountName,
    first_name: String,
    last_name: String,
    street: String,
    city: String,
    state: String,
    zip: u32,
) {
    require_auth(account);

    let code = current_receiver();
    let addresses = Address::table(code, code);

    let itr = addresses
        .find(account)
        .assert("Address for account not found");

    let mut address = itr.get().assert("read");
    address.first_name = first_name;
    address.last_name = last_name;
    address.street = street;
    address.city = city;
    address.state = state;
    address.zip = zip;

    itr.modify(account, &address).assert("write");
}

#[eosio_action]
fn remove(account: AccountName) {
    require_auth(account);

    let code = current_receiver();
    let addresses = Address::table(code, code);

    let itr = addresses
        .find(account)
        .assert("Address for account not found");

    itr.erase().assert("read");
}

#[eosio_action]
fn like(account: AccountName) {
    let code = current_receiver();
    let addresses = Address::table(code, code);

    let itr = addresses
        .find(account)
        .assert("Address for account not found");

    let mut address = itr.get().assert("read");
    address.liked += 1;
    itr.modify(address.account, &address).assert("write");
}

#[eosio_action]
fn likezip(zip: u32) {
    let code = current_receiver();

    let zip_index = Address::zip(code, code);
    for cursor in zip_index.lower_bound(&zip).iter() {
        let mut addr = cursor.get().assert("read");
        if addr.zip != zip {
            break;
        }
        addr.liked += 1;
        cursor.modify(0, &addr).assert("write");
    }
}

eosio_abi!(add, update, remove, like, likezip);

#[eosio_table(address)]
struct Address {
    #[primary]
    account: AccountName,
    first_name: String,
    last_name: String,
    street: String,
    city: String,
    state: String,
    #[secondary]
    zip: u32,
    liked: u64,
}
