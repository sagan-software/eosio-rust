mod build_contracts;
mod build_docs;
mod deploy_examples;
mod docker_init;
mod docker_tests;
mod docker_up;
mod opts;
mod run_examples;

use self::opts::{Cmd, Opt};
use std::process::Command;
use structopt::StructOpt;
use util::{build_contract, docker_compose, RunOr};

fn main() {
    // std::env::set_var("RUST_LOG", "info");
    // pretty_env_logger::init();
    let opt = Opt::from_args();
    match opt.cmd {
        Cmd::RunBench => {
            Command::new("cargo")
                .arg("build")
                .arg("--release")
                .arg("-p")
                .arg("bench")
                .run_or_panic();
            build_contract("bench_rs");
            docker_compose()
                .arg("exec")
                .arg("keosd")
                .arg("/mnt/dev/project/target/release/bench")
                .run_or_panic();
        }
        Cmd::BuildContracts(opts) => build_contracts::build_contracts(opts),
        Cmd::BuildDocs => build_docs::build_docs().unwrap(),
        Cmd::RunExamples => run_examples::run_examples(),
        Cmd::RunTests(opts) => docker_tests::run_test(opts),
        Cmd::DockerUp => docker_up::run_docker_up(),
        Cmd::InitChain => docker_compose()
            .arg("exec")
            .arg("keosd")
            .arg("/mnt/dev/project/target/release/scripts")
            .arg("docker-init")
            .run_or_panic(),
        Cmd::DockerInit => docker_init::docker_init(),
        Cmd::DeployExamples => deploy_examples::run_deploy_examples().unwrap(),
    }
}
