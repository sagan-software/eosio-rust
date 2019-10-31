use std::fs::remove_file;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

pub fn cleos() -> Command {
    let mut cmd = Command::new("docker");
    cmd.args(&[
        "exec",
        "docker_keosd_1",
        "cleos",
        "--url",
        "http://nodeosd:8888",
        "--wallet-url",
        "http://127.0.0.1:8900",
    ]);
    cmd
}

pub fn get_target_dir() -> io::Result<PathBuf> {
    let mut exe = std::env::current_exe()?;
    exe.pop();
    exe.pop();
    Ok(exe.join("wasm32-unknown-unknown").join("release"))
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

pub fn push_action(
    account: &str,
    action: &str,
    data: &str,
    auth: &str,
) -> io::Result<ExitStatus> {
    cleos()
        .arg("push")
        .arg("action")
        .arg(account)
        .arg(action)
        .arg(data)
        .arg("-p")
        .arg(auth)
        .status()
}
