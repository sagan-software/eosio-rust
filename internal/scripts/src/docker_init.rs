use std::{process::Command, thread::sleep, time::Duration};
use util::{cleos, push_action, RunOr};

const PUBKEY: &str = "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV";
const PRIVKEY: &str = "5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3";

fn create_wallet() {
    println!("Creating wallet");
    cleos()
        .arg("wallet")
        .arg("create")
        .arg("--to-console")
        .run_or_panic();
    cleos()
        .arg("wallet")
        .arg("import")
        .arg("--private-key")
        .arg(PRIVKEY)
        .run_or_panic()
}

fn create_account(name: &str) {
    println!("Creating system account {}", name);
    cleos()
        .arg("create")
        .arg("account")
        .arg("eosio")
        .arg(name)
        .arg(PUBKEY)
        .arg(PUBKEY)
        .run_or_panic()
}

fn new_account(name: &str) {
    util::new_account(name, PUBKEY, "100.0000 EOS", "100.0000 EOS", "1000")
}

fn set_contract(name: &str) {
    set_contract_with_path(name, name)
}

fn set_contract_with_path(name: &str, path: &str) {
    println!("Setting contract {} ({})", name, path);
    cleos()
        .arg("set")
        .arg("contract")
        .arg(name)
        .arg(format!("/eosio.contracts/build/contracts/{}", path))
        .run_or_panic()
}

fn set_old_contract_with_path(name: &str, path: &str) {
    println!("Setting old contract {} ({})", name, path);
    cleos()
        .arg("set")
        .arg("contract")
        .arg(name)
        .arg(format!("/eosio.contracts/build/old_contracts/{}", path))
        .run_or_none();
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

fn activate_feature(id: &str) {
    let json = format!("[ \"{}\" ]", id);
    push_action("eosio", "activate", &json, "eosio");
}

/// https://developers.eos.io/welcome/latest/tutorials/bios-boot-sequence
pub fn docker_init() {
    create_wallet();
    println!("Creating system accounts");
    for account in SYSTEM_ACCOUNTS {
        create_account(account);
    }
    println!("Setting base system contracts");
    set_contract("eosio.token");
    set_contract("eosio.msig");
    set_contract("eosio.wrap");
    println!("Creating EOS token");
    push_action(
        "eosio.token",
        "create",
        "[ \"eosio\", \"1000000000.0000 EOS\" ]",
        "eosio.token",
    );
    push_action(
        "eosio.token",
        "issue",
        "[ \"eosio\", \"1000000000.0000 EOS\", \"memo\" ]",
        "eosio",
    );
    {
        let mut cmd = Command::new("curl");
        cmd.args(&[
            "-X",
            "POST",
            "http://nodeosd:8888/v1/producer/schedule_protocol_feature_activations",
            "-d",
            r#"{"protocol_features_to_activate":
    ["0ec7e080177b2c02b278d5088611686b49d739925a92d9bfcacd7fc6b74053bd"]}"#,
        ]);
        cmd.status().unwrap();
        sleep(Duration::from_secs(4));
    }
    println!("Setting core system contracts");
    // set_contract_with_path("eosio", "eosio.bios")?;
    // sleep(Duration::from_secs(3));
    set_old_contract_with_path("eosio", "eosio.system");
    sleep(Duration::from_secs(4));
    for id in &[
        // GET_SENDER
        "f0af56d2c5a48d60a4a5b5c903edfb7db3a736a94ed589d0b797df33ff9d3e1d",
        // FORWARD_SETCODE
        "2652f5f96006294109b3dd0bbde63693f55324af452b799ee137a81a905eed25",
        // ONLY_BILL_FIRST_AUTHORIZER
        "8ba52fe7a3956c5cd3a656a3174b931d3bb2abb45578befc59f283ecd816a405",
        // RESTRICT_ACTION_TO_SELF
        "ad9e3d8f650687709fd68f4b90b41f7d825a365b02c23a636cef88ac2ac00c43",
        // DISALLOW_EMPTY_PRODUCER_SCHEDULE
        "68dcaa34c0517d19666e6b33add67351d8c5f69e999ca1e37931bc410a297428",
        // FIX_LINKAUTH_RESTRICTION
        "e0fb64b1085cc5538970158d05a009c24e276fb94e1a0bf6a528b48fbc4ff526",
        // REPLACE_DEFERRED
        "ef43112c6543b88db2283a2e077278c315ae2c84719a8b25f25cc88565fbea99",
        // NO_DUPLICATE_DEFERRED_ID
        "4a90c00d55454dc5b059055ca213579c6ea856967712a56017487886a4d4cc0f",
        // ONLY_LINK_TO_EXISTING_PERMISSION
        "1a99a59d87e06e09ec5b028a9cbb7749b4a5ad8819004365d02dc4379a8b7241",
        // RAM_RESTRICTIONS
        "4e7bf348da00a945489b2a681749eb56f5de00b900014e137ddae39f48f69d67",
        // WEBAUTHN_KEY
        "4fca8bd82bbd181e714e283f83e1b45d95ca5af40fb89ad3977b653c448f78c2",
        // WTMSIG_BLOCK_SIGNATURES
        "299dcb6af692324b899b39f16d5a530a33062804e41f09dc97e9f156b4476707",
    ] {
        activate_feature(id);
    }
    sleep(Duration::from_secs(1));

    set_contract_with_path("eosio", "eosio.system");
    sleep(Duration::from_secs(3));
    push_action("eosio", "setpriv", "[ \"eosio.msig\", 1 ]", "eosio@active");
    push_action("eosio", "setpriv", "[ \"eosio.wrap\", 1 ]", "eosio@active");
    push_action("eosio", "init", "[ 0, \"4,EOS\" ]", "eosio");
    println!("Creating additional accounts");
    util::new_account(
        "ethereos",
        PUBKEY,
        "200.0000 EOS",
        "200.0000 EOS",
        "5000",
    );
    for account in EXAMPLE_ACCOUNTS {
        new_account(account);
        push_action(
            "eosio.token",
            "transfer",
            format!(
                "[ \"eosio\", \"{}\", \"100.0000 EOS\", \"memo\" ]",
                account
            )
            .as_str(),
            "eosio",
        );
    }
}
