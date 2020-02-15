#[eosio::action]
fn hi(name: eosio::AccountName) {
    eosio_cdt::print!("Hello, ", name, "!");
}

eosio::abi!(hi);
