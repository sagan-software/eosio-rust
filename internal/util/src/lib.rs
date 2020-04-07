use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::remove_file,
    io,
    path::{Path, PathBuf},
    process::Command,
};

pub trait RunOr {
    fn run_or_none(&mut self) -> Option<()>;
    fn run_or_panic(&mut self);
}

impl RunOr for Command {
    fn run_or_none(&mut self) -> Option<()> {
        let status = self.status().expect("failed to execute process");
        if status.success() {
            Some(())
        } else {
            None
        }
    }

    fn run_or_panic(&mut self) {
        if self.run_or_none().is_none() {
            panic!("Failed to run command: {:?}", self);
        }
    }
}

pub fn docker_compose() -> Command {
    let mut cmd = Command::new("docker-compose");
    let docker_compose_yml = get_project_dir()
        .unwrap()
        .join("docker")
        .join("docker-compose.yml");
    cmd.arg("-f");
    cmd.arg(docker_compose_yml);
    cmd
}

pub fn cleos() -> Command {
    let mut cmd = Command::new("cleos");
    cmd.args(&[
        "--url",
        "http://nodeosd:8888",
        "--wallet-url",
        "http://127.0.0.1:8900",
    ]);
    cmd
}

pub fn get_project_dir() -> io::Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path.pop();
    path.pop();
    path.pop();
    Ok(path)
}

pub fn get_target_dir() -> io::Result<PathBuf> {
    Ok(get_project_dir()?
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release"))
}

pub fn remove_file_if_exists<P: AsRef<Path>>(path: P) -> io::Result<()> {
    match remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(err)
            }
        }
    }
}

pub fn push_action_cmd(
    account: impl AsRef<str>,
    action: impl AsRef<str>,
    data: impl AsRef<str>,
    auth: impl AsRef<str>,
) -> Command {
    let account = account.as_ref();
    let action = action.as_ref();
    let data = data.as_ref();
    let auth = auth.as_ref();
    log::debug!("push action {} {} '{}' -p {}", account, action, data, auth);
    let mut cmd = cleos();
    cmd.arg("push")
        .arg("action")
        .arg(account)
        .arg(action)
        .arg(data)
        .arg("-p")
        .arg(auth);
    cmd
}

pub fn push_action(
    account: impl AsRef<str>,
    action: impl AsRef<str>,
    data: impl AsRef<str>,
    auth: impl AsRef<str>,
) {
    push_action_cmd(account, action, data, auth).run_or_panic()
}

pub fn push_action_opt(
    account: impl AsRef<str>,
    action: impl AsRef<str>,
    data: impl AsRef<str>,
    auth: impl AsRef<str>,
) -> Option<()> {
    push_action_cmd(account, action, data, auth).run_or_none()
}

lazy_static! {
    static ref METRICS_RE: Regex =
        Regex::new(r"(?P<bytes>\d+) bytes  (?P<time>\d+) us")
            .expect("bad regex");
}

#[derive(Debug)]
pub struct Metrics {
    pub bytes: u64,
    pub time: u64,
}

fn run_metrics(mut cmd: Command) -> Metrics {
    let out = cmd.output().expect("failed to execute process");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let caps = match METRICS_RE.captures(&stderr) {
        Some(caps) => caps,
        None => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            panic!(
                "No regex matches found, something probably went wrong. \
                 stderr:\n{}\n\nstdout:\n{}",
                stderr, stdout
            );
        }
    };
    let bytes = &caps["bytes"].parse::<u64>().expect("bytes is not a u64");
    let time = &caps["time"].parse::<u64>().expect("time is not a u64");
    Metrics {
        bytes: *bytes,
        time: *time,
    }
}

pub fn push_action_metrics(
    account: impl AsRef<str>,
    action: impl AsRef<str>,
    data: impl AsRef<str>,
    auth: impl AsRef<str>,
) -> Metrics {
    let cmd = push_action_cmd(account, action, data, auth);
    run_metrics(cmd)
}

pub fn set_abi(account: impl AsRef<str>, abi_path: impl AsRef<str>) {
    let account = account.as_ref();
    let abi_path = abi_path.as_ref();
    log::debug!("set abi {} {}", account, abi_path);
    cleos()
        .arg("set")
        .arg("abi")
        .arg(account)
        .arg(abi_path)
        .run_or_panic();
}

pub fn set_code_cmd(
    account: impl AsRef<str>,
    wasm_path: impl AsRef<str>,
) -> Command {
    let account = account.as_ref();
    let wasm_path = wasm_path.as_ref();
    log::debug!("set code {} {}", account, wasm_path);
    let mut cmd = cleos();
    cmd.arg("set").arg("code").arg(account).arg(wasm_path);
    cmd
}

pub fn set_code(account: impl AsRef<str>, wasm_path: impl AsRef<str>) {
    set_code_cmd(account, wasm_path).run_or_panic();
}

pub fn set_code_metrics(
    account: impl AsRef<str>,
    wasm_path: impl AsRef<str>,
) -> Metrics {
    run_metrics(set_code_cmd(account, wasm_path))
}

pub fn set_contract(
    account: impl AsRef<str>,
    abi_path: impl AsRef<str>,
    wasm_path: impl AsRef<str>,
) {
    set_abi(&account, abi_path);
    set_code(account, wasm_path);
}

pub fn new_account(
    name: impl AsRef<str>,
    pubkey: impl AsRef<str>,
    net: impl AsRef<str>,
    cpu: impl AsRef<str>,
    ram: impl AsRef<str>,
) {
    let name = name.as_ref();
    let pubkey = pubkey.as_ref();
    let net = net.as_ref();
    let cpu = cpu.as_ref();
    let ram = ram.as_ref();
    log::debug!("Creating new account {}", name);
    cleos()
        .arg("system")
        .arg("newaccount")
        .arg("eosio")
        .arg("--transfer")
        .arg(name)
        .arg(pubkey)
        .arg("--stake-net")
        .arg(net)
        .arg("--stake-cpu")
        .arg(cpu)
        .arg("--buy-ram-kbytes")
        .arg(ram)
        .run_or_panic()
}

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

pub fn build_contract(package: impl AsRef<str>) {
    let package = package.as_ref();
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
