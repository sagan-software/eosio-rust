use eosio::*;

#[eosio::action]
fn hi(name: AccountName) {
    eosio::print!("Hi, ", name);
}

eosio::abi!(hi);
