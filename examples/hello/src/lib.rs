#[eosio::action]
fn hi(name: eosio::AccountName) {
    eosio::print!("Hello, ", name, "!");
}

eosio::abi!(hi);
