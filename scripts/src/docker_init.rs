use crate::shared::{cleos, push_action};
use std::{io, process::ExitStatus};

const PUBKEY: &str = "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV";
const PRIVKEY: &str = "5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3";

fn create_wallet() -> io::Result<ExitStatus> {
    cleos()
        .arg("wallet")
        .arg("create")
        .arg("--to-console")
        .status()?;
    cleos()
        .arg("wallet")
        .arg("import")
        .arg("--private-key")
        .arg(PRIVKEY)
        .status()
}

fn create_account(name: &str) -> io::Result<ExitStatus> {
    cleos()
        .arg("create")
        .arg("account")
        .arg("eosio")
        .arg(name)
        .arg(PUBKEY)
        .arg(PUBKEY)
        .status()
}

fn new_account(name: &str) -> io::Result<ExitStatus> {
    cleos()
        .arg("system")
        .arg("newaccount")
        .arg("eosio")
        .arg("--transfer")
        .arg(name)
        .arg(PUBKEY)
        .arg("--stake-net")
        .arg("100.0000 EOS")
        .arg("--stake-cpu")
        .arg("100.0000 EOS")
        .arg("--buy-ram-kbytes")
        .arg("1000")
        .status()
}

fn set_contract(name: &str) -> io::Result<ExitStatus> {
    set_contract_with_path(name, name)
}

fn set_contract_with_path(name: &str, path: &str) -> io::Result<ExitStatus> {
    cleos()
        .arg("set")
        .arg("contract")
        .arg(name)
        .arg(format!("eosio.contracts/build/contracts/{}", path))
        .status()
}

const SYSTEM_ACCOUNTS: &[&str] = &[
    "eosio.bpay",
    "eosio.msig",
    "eosio.names",
    "eosio.ram",
    "eosio.ramfee",
    "eosio.saving",
    "eosio.stake",
    "eosio.token",
    "eosio.vpay",
    "eosio.rex",
    "eosio.wrap",
];

const EXAMPLE_ACCOUNTS: &[&str] = &[
    "hello",
    "hellobare",
    "hellocpp",
    "tictactoe",
    "alice",
    "bob",
    "carol",
    "dan",
    "addressbook",
    "eosiotkncpp",
];

pub fn run_docker_init() -> io::Result<()> {
    create_wallet()?;
    for account in SYSTEM_ACCOUNTS {
        create_account(account)?;
    }
    set_contract("eosio.token")?;
    set_contract("eosio.msig")?;
    set_contract("eosio.wrap")?;
    push_action(
        "eosio.token",
        "create",
        "[ \"eosio\", \"1000000000.0000 EOS\" ]",
        "eosio.token",
    )?;
    push_action(
        "eosio.token",
        "issue",
        "[ \"eosio\", \"1000000000.0000 EOS\", \"memo\" ]",
        "eosio",
    )?;
    set_contract_with_path("eosio", "eosio.system")?;
    push_action("eosio", "setpriv", "[ \"eosio.msig\", 1 ]", "eosio@active")?;
    push_action("eosio", "setpriv", "[ \"eosio.wrap\", 1 ]", "eosio@active")?;
    push_action("eosio", "init", "[ 0, \"4,EOS\" ]", "eosio")?;
    for account in EXAMPLE_ACCOUNTS {
        new_account(account)?;
    }
    Ok(())
}
