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

    let _self = AccountName::receiver();
    let table = Address::table(_self, _self);

    table
        .find(account)
        .is_none()
        .assert("Address for account already exists");

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
    table.emplace(account, &address).assert("write");
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

    let _self = AccountName::receiver();
    let table = Address::table(_self, _self);
    let cursor = table.find(account).assert("Address for account not found");

    let mut address = cursor.get().assert("read");
    address.first_name = first_name;
    address.last_name = last_name;
    address.street = street;
    address.city = city;
    address.state = state;
    address.zip = zip;

    cursor.modify(None, &address).assert("write");
}

#[eosio_action]
fn erase(account: AccountName) {
    require_auth(account);

    let _self = AccountName::receiver();
    let addresses = Address::table(_self, _self);
    let cursor = addresses
        .find(account)
        .assert("Address for account not found");

    cursor.erase().assert("read");
}

#[eosio_action]
fn like(account: AccountName) {
    let _self = AccountName::receiver();
    let addresses = Address::table(_self, _self);
    let cursor = addresses
        .find(account)
        .assert("Address for account not found");

    let mut address = cursor.get().assert("read");
    address.liked += 1;
    cursor
        .modify(Some(address.account), &address)
        .assert("write");
}

#[eosio_action]
fn likezip(zip: u32) {
    let _self = AccountName::receiver();
    let table = Address::zip(_self, _self);
    for cursor in table.lower_bound(zip).into_iter() {
        let mut addr = cursor.get().assert("read");
        if addr.zip != zip {
            break;
        }
        addr.liked += 1;
        cursor.modify(None, &addr).assert("write");
    }
}

eosio_abi!(add, update, erase, like, likezip);

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
