use std::{fs, process::Command};
use util::{get_project_dir, get_target_dir, RunOr};

fn main() {
    println!("Building bench.cpp");

    let build_dir = get_target_dir().unwrap();
    fs::create_dir_all(&build_dir).unwrap();

    let mut src_dir = get_project_dir().unwrap();
    src_dir.push(file!());
    src_dir.pop();

    let container = "bench_cpp_builder";

    let container_exists = Command::new("docker")
        .arg("container")
        .arg("inspect")
        .arg(container)
        .run_or_none()
        .is_some();

    if container_exists {
        Command::new("docker")
            .arg("kill")
            .arg(container)
            .run_or_panic();
        // Command::new("docker")
        //     .arg("rm")
        //     .arg(container)
        //     .run_or_panic();
    }

    Command::new("docker")
        .arg("run")
        .arg(format!("--name={}", container))
        .arg(format!("--volume={}:/src:ro", src_dir.to_str().unwrap()))
        .arg(format!("--volume={}:/build", build_dir.to_str().unwrap()))
        .arg("--detach")
        .arg("--entrypoint=sleep")
        .arg("--rm")
        .arg("sagansoftware/eosio.cdt:1.7.0")
        .arg("30")
        .run_or_panic();

    Command::new("docker")
        .arg("exec")
        .arg(container)
        .arg("eosio-cpp")
        .arg("-o=/tmp/bench_cpp.wasm")
        .arg("-abigen")
        .arg("-abigen_output=/tmp/bench.json")
        .arg("-contract=bench")
        .arg("/src/contract.cpp")
        .run_or_panic();

    Command::new("docker")
        .arg("exec")
        .arg(container)
        .arg("mv")
        .arg("/tmp/bench_cpp.wasm")
        .arg("/build")
        .run_or_panic();

    Command::new("docker")
        .arg("exec")
        .arg(container)
        .arg("mv")
        .arg("/tmp/bench_cpp.abi")
        .arg("/build/bench.abi.json")
        .run_or_panic();

    Command::new("docker")
        .arg("kill")
        .arg(container)
        .run_or_panic();
}
