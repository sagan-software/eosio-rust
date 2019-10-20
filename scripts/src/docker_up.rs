use crate::opts::BuildCmd;
use crate::shared::{get_target_dir, remove_file_if_exists};
use std::fs::canonicalize;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

fn docker_compose() -> Command {
    let mut cmd = Command::new("docker-compose");
    let yaml = Path::new(".").join("docker").join("docker-compose.yml");
    cmd.arg("--file").arg(yaml);
    cmd
}

fn docker_volume(name: &str) -> io::Result<ExitStatus> {
    Command::new("docker")
        .arg("volume")
        .arg("rm")
        .arg(name)
        .status()?;
    Command::new("docker")
        .arg("volume")
        .arg("create")
        .arg(format!("--name={}", name))
        .status()
}

pub fn run_docker_up() {
    docker_compose().arg("down").status().unwrap();
    docker_volume("nodeos-data-volume").unwrap();
    docker_volume("keosd-data-volume").unwrap();
    docker_compose().arg("up").status().unwrap();
}
