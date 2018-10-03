#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::prelude::*;
use std::marker::PhantomData;

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
    let addresses = Address::table(code, code, n!(address));

    let itr = addresses.find(account);
    eosio_assert!(addresses.is_end(&itr), "Address for account already exists");

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
    addresses.emplace(account, address);

    // let table_name = (n!(address) & 18446744073709551600) | (0 & 15);
    let table_name = (n!(address) & 0xFFFFFFFFFFFFFFF0u64) | (0 & 0x000000000000000Fu64);
    let zipptr: *const u64 = &(zip as u64);
    unsafe {
        ::eosio::sys::db_idx64_store(code.into(), table_name, code.into(), account.into(), zipptr);
    }
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
    let addresses = Address::table(code, code, n!(address));

    let itr = addresses.find(account);
    eosio_assert!(!addresses.is_end(&itr), "Address for account not found");

    let mut address = itr.get().unwrap();
    address.first_name = first_name;
    address.last_name = last_name;
    address.street = street;
    address.city = city;
    address.state = state;
    address.zip = zip;

    addresses.modify(&itr, account, address);
}

#[eosio_action]
fn remove(account: AccountName) {
    require_auth(account);

    let code = current_receiver();
    let addresses = Address::table(code, code, n!(address));

    let itr = addresses.find(account);
    eosio_assert!(!addresses.is_end(&itr), "Address for account not found");

    itr.erase();
}

#[eosio_action]
fn like(account: AccountName) {
    let code = current_receiver();
    let addresses = Address::table(code, code, n!(address));

    let itr = addresses.find(account);
    eosio_assert!(!addresses.is_end(&itr), "Address for account not found");
    //eosio_assert!(!itr.is_end(), "Address for account not found");

    let mut address = itr.get().unwrap();
    address.liked += 1;
    addresses.modify(&itr, address.account, address);
    //itr.modify(address.account, address);
}

#[eosio_action]
fn likezip(zip: u32) {
    let code = current_receiver();
    let addresses = Address::table(code, code, n!(address));
    // Address::zip::find
    let secondary: *mut u64 = &mut (zip.clone() as u64);
    // let secondary: *mut u64 = &mut 9999999999u64;
    let primary: *mut u64 = &mut 0u64;
    // let table_name = (n!(address) & 18446744073709551600) | (0 & 15);
    let table_name = (n!(address) & 0xFFFFFFFFFFFFFFF0u64) | (0 & 0x000000000000000Fu64);
    // let table_name = n!(address2);
    // let table_name = n!(balls);
    let itr = unsafe {
        ::eosio::sys::db_idx64_lowerbound(code.into(), code.into(), table_name, secondary, primary)
    };
    " ||| \0".print();
    itr.print();
    " ||| \0".print();
    let secondary = unsafe { *secondary };
    secondary.print();
    " !!! \0".print();
    let primary = unsafe { *primary };
    primary.print();
    // " ???? \0".print();
    // Address::by_zip().lowerbound();
    // " ???? \0".print();
    // let s: SecondaryIndex<f64, Address> = SecondaryIndex {
    //     _key_type: PhantomData,
    //     _row_type: PhantomData,
    // };
    // s.lowerbound();

    // let zip_index = Address::by_zip(code, code, n!(address));
    // let itr = zip_index.lower_bound(zip);
    // for address in itr {
    //     address.liked += 1;
    //     itr.modify(None, address);
    // }
}

eosio_abi!(add, update, remove, like, likezip);

#[derive(TableRow, Read, Write)]
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

// impl TableRow for Address {
//     fn primary_key(&self) -> u64 {
//         self.account.into()
//     }

//     fn secondary_keys(&self) -> [Option<SecondaryKey>; 16] {
//         [
//             Some(SecondaryKey::U64(u64::from(self.zip))),
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//         ]
//     }
// }

// impl Address {
//     fn by_zip() -> SecondaryIndex<u64, Self> {
//         SecondaryIndex {
//             _key_type: PhantomData,
//             _row_type: PhantomData,
//         }
//     }
// }
