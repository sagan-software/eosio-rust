use crate::opts::BuildContracts;
use std::{path::Path, process::Command};
use util::{get_target_dir, remove_file_if_exists, RunOr};

fn cargo_build(package: &str) {
    println!("building package: {}", package);
    Command::new("cargo")
        .env("RUSTFLAGS", "-C link-args=-zstack-size=48000")
        .arg("build")
        .arg("--release")
        .arg("--target=wasm32-unknown-unknown")
        .arg("-p")
        .arg(package)
        .run_or_panic()
}

fn wasm_gc<I: AsRef<Path>, O: AsRef<Path>>(input: I, output: O) {
    println!(
        "running wasm-gc (input: {:#?}, output: {:#?})",
        input.as_ref(),
        output.as_ref()
    );
    Command::new("wasm-gc")
        .arg(input.as_ref())
        .arg(output.as_ref())
        .run_or_panic()
}

// fn wasm_opt<I: AsRef<Path>, O: AsRef<Path>>(input: I, output: O) {
//     println!(
//         "running wasm-opt (input: {:#?}, output: {:#?})",
//         input.as_ref(),
//         output.as_ref()
//     );
//     Command::new("wasm-opt")
//         .arg("-Oz")
//         .arg("--output")
//         .arg(output.as_ref())
//         .arg(input.as_ref())
//         .run_or_panic()
// }

// TODO: requires wabt
// fn wasm2wat<I: AsRef<Path>, O: AsRef<Path>>(input: I, output: O) {
//     println!(
//         "running wasm2wat (input: {:#?}, output: {:#?})",
//         input.as_ref(),
//         output.as_ref()
//     );
//     Command::new("wasm2wat")
//         .arg(input.as_ref())
//         .arg("-o")
//         .arg(output.as_ref())
//         .arg("--generate-names")
//         .run_or_panic();
// }

pub fn build_contract(package: &str) {
    cargo_build(package);
    let target_dir = get_target_dir().expect("failed to get target directory");
    let bin = package.replace('-', "_");
    let wasm = target_dir.join(format!("{}.wasm", bin));
    let gc_wasm = target_dir.join(format!("{}_gc.wasm", bin));
    // let gc_opt_wasm = target_dir.join(format!("{}_gc_opt.wasm", bin));
    let gc_opt_wat = target_dir.join(format!("{}_gc_opt.wat", bin));
    remove_file_if_exists(&gc_wasm).unwrap_or_else(|e| {
        panic!("failed to remove {:#?}: {:#?}", gc_wasm, e)
    });
    // remove_file_if_exists(&gc_opt_wasm)?;
    remove_file_if_exists(&gc_opt_wat).unwrap_or_else(|e| {
        panic!("failed to remove {:#?}: {:#?}", gc_opt_wat, e)
    });
    wasm_gc(wasm, &gc_wasm);
    // These two commands require binaryen:
    // wasm_opt(gc_wasm, &gc_opt_wasm);
    // wasm2wat(gc_opt_wasm, gc_opt_wat);
}

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
