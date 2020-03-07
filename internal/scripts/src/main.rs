mod build_contracts;
mod build_docs;
mod deploy_examples;
mod docker_init;
mod docker_tests;
mod docker_up;
mod opts;
mod run_examples;

use self::opts::{Cmd, Opt};
use structopt::StructOpt;

fn main() {
    // std::env::set_var("RUST_LOG", "info");
    // pretty_env_logger::init();
    let opt = Opt::from_args();
    match opt.cmd {
        Cmd::RunBench => (),
        Cmd::BuildContracts(opts) => build_contracts::build_contracts(opts),
        Cmd::BuildDocs => build_docs::build_docs().unwrap(),
        Cmd::RunExamples => run_examples::run_examples(),
        Cmd::RunTests(opts) => docker_tests::run_test(opts),
        Cmd::DockerUp => docker_up::run_docker_up(),
        Cmd::DockerInit => docker_init::docker_init(),
        Cmd::DeployExamples => deploy_examples::run_deploy_examples().unwrap(),
    }
}
