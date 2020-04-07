use std::{io, process::ExitStatus};
use util::{build_contract, cleos};

fn deploy_example_contract(account: &str, bin: &str) -> io::Result<ExitStatus> {
    cleos()
        .arg("set")
        .arg("abi")
        .arg(account)
        .arg(format!(
            "/mnt/dev/project/examples/{}/{}.abi.json",
            bin, bin
        ))
        .status()?;
    cleos()
        .arg("set")
        .arg("code")
        .arg(account)
        .arg(format!(
            "/mnt/dev/project/target/wasm32-unknown-unknown/release/{}_gc.wasm",
            bin
        ))
        .status()
}

pub fn run_deploy_examples() -> io::Result<()> {
    for (package, bin, account) in &[
        ("addressbook", "addressbook", "addressbook"),
        ("hello", "hello", "hello"),
        ("hello_bare", "hello_bare", "hellobare"),
        ("tictactoe", "tictactoe", "tictactoe"),
    ] {
        build_contract(package);
        deploy_example_contract(account, bin)?;
    }
    Ok(())
}
