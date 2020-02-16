use crate::shared::project_dir;
use std::{
    io,
    path::Path,
    process::{Command, ExitStatus},
};

fn build_readme<T: AsRef<Path>>(crate_name: T) -> io::Result<ExitStatus> {
    let crate_name = crate_name.as_ref();
    let crate_root = project_dir()?.join("crates").join(crate_name);
    let output = crate_root.join("README.md");
    println!("building README.md for {:?} ({:?})", crate_root, output);
    Command::new("cargo")
        .arg("readme")
        .arg("--project-root")
        .arg(crate_root)
        .arg("--output")
        .arg(output)
        .status()
}

fn symlink_licenses<T: AsRef<Path>>(crate_name: T) -> io::Result<()> {
    let crate_name = crate_name.as_ref();
    let crate_root = project_dir()?.join("crates").join(crate_name);
    for license in &["LICENSE-APACHE", "LICENSE-MIT"] {
        let src = Path::new("..").join("..").join(license);
        let dst = crate_root.join(license);
        println!("Creating symlink: src={:?}, dst={:?}", src, dst);
        if dst.exists() {
            std::fs::remove_file(&dst)?;
        }
        #[cfg(target_os = "linux")]
        {
            std::os::unix::fs::symlink(src, dst)?;
        }
        #[cfg(target_os = "windows")]
        {
            std::os::windows::fs::symlink_file(src, dst)?;
        }
    }
    Ok(())
}

const CRATE_NAMES: &[&str] = &[
    "eosio",
    "eosio_cdt",
    "eosio_cdt_sys",
    "eosio_macros",
    "eosio_macros_internal",
    "eosio_numstr",
];

pub fn build_docs() -> io::Result<()> {
    println!("building docs");
    Command::new("cargo")
        .arg("doc")
        .arg("--all")
        .arg("--all-features")
        .arg("--no-deps")
        .status()?;
    for crate_name in CRATE_NAMES {
        symlink_licenses(crate_name)?;
        build_readme(crate_name)?;
    }
    Ok(())
}
