use crate::shared::project_dir;
use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus};

fn build_readme<T: AsRef<Path>>(crate_name: T) -> io::Result<ExitStatus> {
    let crate_name = crate_name.as_ref();
    let project_root = project_dir()?.join("crates").join(crate_name);
    let output = project_root.join("README.md");
    println!("building README.md for {:?} ({:?})", project_root, output);
    Command::new("cargo")
        .arg("readme")
        .arg("--project-root")
        .arg(project_root)
        .arg("--output")
        .arg(output)
        .status()
}

pub fn build_docs() -> io::Result<()> {
    println!("building docs");
    Command::new("cargo")
        .arg("doc")
        .arg("--all")
        .arg("--all-features")
        .arg("--no-deps")
        .status()?;
    build_readme("eosio_numstr")?;
    Ok(())
}
