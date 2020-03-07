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
    cmd.arg("-f");
    cmd.arg("docker/docker-compose.yml");
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
    println!("push action {} {} '{}' -p {}", account, action, data, auth);
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
    println!("set abi {} {}", account, abi_path);
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
    println!("set code {} {}", account, wasm_path);
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
    println!("Creating new account {}", name);
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
