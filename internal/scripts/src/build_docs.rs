use std::{
    fs, io, os,
    path::{Path, PathBuf},
    process::Command,
};
use util::{get_project_dir, RunOr};

fn build_readme(crate_root: &Path) -> io::Result<()> {
    let output = crate_root.join("README.md");
    println!("building README.md for {:?} ({:?})", crate_root, output);
    Command::new("cargo")
        .arg("readme")
        .arg("--project-root")
        .arg(crate_root)
        .arg("--output")
        .arg(output)
        .run_or_panic();
    Ok(())
}

fn symlink_licenses(crate_root: &Path) -> io::Result<()> {
    for license in &["LICENSE-APACHE", "LICENSE-MIT"] {
        let src = Path::new("..").join("..").join(license);
        let dst = crate_root.join(license);
        println!("Creating symlink: src={:?}, dst={:?}", src, dst);
        if dst.exists() {
            fs::remove_file(&dst)?;
        }
        #[cfg(target_os = "linux")]
        {
            os::unix::fs::symlink(src, dst)?;
        }
        #[cfg(target_os = "windows")]
        {
            os::windows::fs::symlink_file(src, dst)?;
        }
    }
    Ok(())
}

fn build_docs_for_dir(dir: PathBuf) {
    println!("Building docs for all crates in dir: {:?}", dir);
    for dir_entry in fs::read_dir(dir).unwrap() {
        let dir_entry = dir_entry.unwrap();
        let crate_root = dir_entry.path();
        symlink_licenses(&crate_root).unwrap();
        build_readme(&crate_root).unwrap();
    }
}

pub fn build_docs() -> io::Result<()> {
    println!("building docs");
    let root_dir = get_project_dir().unwrap();
    Command::new("mdbook")
        .arg("build")
        .arg("book")
        .run_or_panic();
    Command::new("cargo")
        .arg("doc")
        .arg("--all")
        .arg("--all-features")
        .arg("--no-deps")
        .run_or_panic();
    build_docs_for_dir(root_dir.join("crates"));
    build_docs_for_dir(root_dir.join("contracts"));
    Ok(())
}
