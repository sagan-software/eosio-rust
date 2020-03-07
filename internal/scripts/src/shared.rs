use std::{
    fs::remove_file,
    io,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

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

pub fn project_dir() -> io::Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path.pop();
    path.pop();
    path.pop();
    Ok(path)
}

pub fn get_target_dir() -> io::Result<PathBuf> {
    Ok(project_dir()?
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
