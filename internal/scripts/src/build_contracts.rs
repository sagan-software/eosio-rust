use crate::opts::BuildContracts;
use util::build_contract;

const ALL: &[&str] = &[
    "addressbook",
    "hello_bare",
    "hello",
    "tictactoe",
    "eosio_bios",
    "eosio_msig",
    "eosio_token",
    "eosio_wrap",
];

pub fn build_contracts(opts: BuildContracts) {
    match opts.package {
        Some(pkg) => {
            build_contract(&pkg);
        }
        None => {
            for pkg in ALL {
                build_contract(pkg);
            }
        }
    }
}
