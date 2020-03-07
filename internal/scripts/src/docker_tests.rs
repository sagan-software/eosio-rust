use crate::{build_contracts::build_contract, opts::RunTestsCmd};
use std::{
    io,
    process::{Command, ExitStatus},
};
use util::get_target_dir;

const CONTRACTS: &[&str] = &["bios", "msig", "token", "wrap"];

fn eosio_contract_tests() -> io::Result<ExitStatus> {
    let target_dir = get_target_dir()?;
    let mut cmd = Command::new("docker");
    cmd.arg("run").arg("--rm");

    for name in CONTRACTS {
        let crate_name = format!("eosio_{}", name);
        let contract_name = format!("eosio.{}", name);
        build_contract(&crate_name);
        let volume = {
            let name = format!("{}_gc.wasm", crate_name);
            let path = target_dir.join(name);
            format!(
                "{}:/eosio.contracts/build/contracts/{}/{}.wasm:ro",
                path.to_string_lossy(),
                contract_name,
                contract_name
            )
        };
        cmd.arg("--volume").arg(volume);
    }

    cmd.arg("--entrypoint")
        .arg("/eosio.contracts/build/tests/unit_test")
        .arg("sagansoftware/eosio.contracts:1.9.0")
        .arg("--show_progress=yes");

    for name in CONTRACTS {
        cmd.arg(format!("--run_test=eosio_{}_tests", name));
    }

    // cmd.arg("--run_test=eosio_system_tests");

    cmd.status()
}

pub fn run_test(_opts: RunTestsCmd) {
    eosio_contract_tests().unwrap();
}
