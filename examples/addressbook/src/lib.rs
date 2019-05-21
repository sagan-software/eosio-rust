use eosio::*;

#[eosio::action]
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

    let _self = current_receiver();
    let table = Address::table(_self, _self);

    table
        .find(account)
        .is_none()
        .check("Address for account already exists");

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
    table.emplace(account, &address).check("write");
}

#[eosio::action]
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

    let _self = current_receiver();
    let table = Address::table(_self, _self);
    let cursor = table.find(account).check("Address for account not found");

    let mut address = cursor.get().check("read");
    address.first_name = first_name;
    address.last_name = last_name;
    address.street = street;
    address.city = city;
    address.state = state;
    address.zip = zip;

    cursor.modify(None, &address).check("write");
}

#[eosio::action]
fn erase(account: AccountName) {
    require_auth(account);

    let _self = current_receiver();
    let addresses = Address::table(_self, _self);
    let cursor = addresses
        .find(account)
        .check("Address for account not found");

    cursor.erase().check("read");
}

#[eosio::action]
fn like(account: AccountName) {
    let _self = current_receiver();
    let addresses = Address::table(_self, _self);
    let cursor = addresses
        .find(account)
        .check("Address for account not found");

    let mut address = cursor.get().check("read");
    address.liked += 1;
    cursor
        .modify(Some(address.account), &address)
        .check("write");
}

#[eosio::action]
fn likezip(zip: u32) {
    let _self = current_receiver();
    let table = Address::zip(_self, _self);
    for cursor in table.lower_bound(zip).into_iter() {
        let mut addr = cursor.get().check("read");
        if addr.zip != zip {
            break;
        }
        addr.liked += 1;
        cursor.modify(None, &addr).check("write");
    }
}

eosio::abi!(add, update, erase, like, likezip);

#[eosio::table(address)]
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
