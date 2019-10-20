use crate::opts::RunTestsCmd;
use crate::shared::get_target_dir;
use path_slash::PathExt;
use std::io;
use std::process::{Command, ExitStatus};

fn cargo_test() -> io::Result<ExitStatus> {
    Command::new("cargo")
        .arg("test")
        .arg("-p")
        .arg("eosio")
        .arg("-p")
        .arg("eosio-macros")
        .arg("-p")
        .arg("eosio-macros-internal")
        .arg("-p")
        .arg("eosio-numstr")
        .status()
}

fn eosio_contract_tests() -> io::Result<ExitStatus> {
    crate::build_contract("eosio-token")?;
    crate::build_contract("eosio-wrap")?;
    let current_dir = std::env::current_dir()?;
    let eosio_token_volume = {
        let path = current_dir
            .join("target")
            .join("wasm32-unknown-unknown")
            .join("release")
            .join("eosio_token_gc_opt.wasm");
        format!(
            "{}:/eosio.contracts/build/contracts/eosio.token/eosio.token.wasm:ro",
            path.to_string_lossy()
        )
    };
    let eosio_wrap_volume = {
        let path = current_dir
            .join("target")
            .join("wasm32-unknown-unknown")
            .join("release")
            .join("eosio_wrap_gc_opt.wasm");
        format!(
            "{}:/eosio.contracts/build/contracts/eosio.wrap/eosio.wrap.wasm:ro",
            path.to_string_lossy()
        )
    };
    Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("--volume")
        .arg(eosio_token_volume)
        .arg("--volume")
        .arg(eosio_wrap_volume)
        .arg("--entrypoint")
        .arg("/eosio.contracts/build/tests/unit_test")
        .arg("sagansoftware/eos:latest")
        .arg("--show_progress=yes")
        .arg("--run_test=eosio_token_tests")
        .arg("--run_test=eosio_wrap_tests")
        .status()
}

pub fn run_test(opts: RunTestsCmd) {
    cargo_test().unwrap();
    eosio_contract_tests().unwrap();
}
