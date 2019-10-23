extern crate scripts;

use scripts::*;
use structopt::StructOpt;

fn main() {
    // std::env::set_var("RUST_LOG", "info");
    // pretty_env_logger::init();
    let opt = Opt::from_args();
    match opt.cmd {
        Cmd::RunBench => run_bench(),
        Cmd::RunBuild(opts) => run_build(opts),
        Cmd::RunExamples => run_examples().unwrap(),
        Cmd::RunTests(opts) => run_test(opts),
        Cmd::DockerUp => run_docker_up(),
        Cmd::DockerInit => run_docker_init().unwrap(),
        Cmd::DeployExamples => run_deploy_examples().unwrap(),
    }
}
