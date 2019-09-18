#[eosio::action]
fn hi(name: eosio::AccountName) {
    eosio::print!("Hi, ", name);
}

eosio::abi!(hi);
