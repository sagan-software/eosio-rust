use std::io;
use std::process::Command;
// use std::fs::Path;

// fn build_readme<P: AsRef<Path>>(project_root: P) -> io::Result<()> {
//     println!("building README.md for {}", pkg);
//     Command::new("cargo")
//         .arg("readme")
//         .arg("--project-root")
//         .arg(project_root)
//         .arg

// }

pub fn build_docs() -> io::Result<()> {
    println!("building docs");
    Command::new("cargo")
        .arg("doc")
        .arg("--all")
        .arg("--all-features")
        .arg("--no-deps")
        .status()?;
    Ok(())
}
