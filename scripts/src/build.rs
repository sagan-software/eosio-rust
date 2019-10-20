use crate::opts::BuildCmd;
use crate::shared::{get_target_dir, remove_file_if_exists};
use std::fs::canonicalize;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

fn cargo_fmt() -> io::Result<ExitStatus> {
    Command::new("cargo").arg("fmt").arg("--all").status()
}

fn cargo_build(package: &str) -> io::Result<ExitStatus> {
    log::info!("Building package '{}'", package);
    Command::new("cargo")
        .env("RUSTFLAGS", "-C link-args=-zstack-size=48000")
        .arg("build")
        .arg("--release")
        .arg("--target=wasm32-unknown-unknown")
        .arg("-p")
        .arg(package)
        .status()
}

fn wasm_gc<I: AsRef<Path>, O: AsRef<Path>>(
    input: I,
    output: O,
) -> io::Result<ExitStatus> {
    Command::new("wasm-gc")
        .arg(canonicalize(input)?)
        .arg(output.as_ref())
        .status()
}

// fn wasm_opt<I: AsRef<Path>, O: AsRef<Path>>(
//     input: I,
//     output: O,
// ) -> io::Result<ExitStatus> {
//     Command::new("wasm-opt")
//         .arg("-Oz")
//         .arg("--output")
//         .arg(output.as_ref())
//         .arg(canonicalize(input)?)
//         .status()
// }

fn wasm2wat<I: AsRef<Path>, O: AsRef<Path>>(
    input: I,
    output: O,
) -> io::Result<ExitStatus> {
    Command::new("wasm2wat")
        .arg(canonicalize(input)?)
        .arg("-o")
        .arg(output.as_ref())
        .arg("--generate-names")
        .status()
}

pub fn build_contract(package: &str) -> io::Result<()> {
    cargo_build(package)?;
    let target_dir = get_target_dir()?;
    let bin = package.replace('-', "_");
    let wasm = target_dir.join(format!("{}.wasm", bin));
    let gc_wasm = target_dir.join(format!("{}_gc.wasm", bin));
    let gc_wat = target_dir.join(format!("{}_gc.wat", bin));
    remove_file_if_exists(&gc_wasm)?;
    // remove_file_if_exists(&gc_opt_wasm)?;
    remove_file_if_exists(&gc_wat)?;
    wasm_gc(wasm, &gc_wasm)?;
    // wasm_opt(gc_wasm, &gc_opt_wasm)?;
    wasm2wat(gc_wasm, gc_wat)?;
    Ok(())
}

const ALL: &[&str] = &[
    "hello-bare",
    "hello",
    "tictactoe",
    "eosio-token",
    "eosio-wrap",
];

pub fn run_build(opts: BuildCmd) {
    match opts.package {
        Some(pkg) => {
            build_contract(&pkg).unwrap();
        }
        None => {
            for pkg in ALL {
                build_contract(pkg).unwrap();
            }
        }
    }
}
