use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "scripts")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Cmd,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "which subcommand to run")]
pub enum Cmd {
    DeployExamples,
    DockerInit,
    DockerUp,
    RunBench,
    BuildContracts(BuildContracts),
    BuildDocs,
    RunExamples,
    RunTests(RunTestsCmd),
}

#[derive(StructOpt, Debug)]
pub struct BuildContracts {
    /// The package to build
    #[structopt(short = "p", long = "package")]
    pub package: Option<String>,
    #[structopt(long = "wasm-opt")]
    pub wasm_opt: bool,
}

#[derive(StructOpt, Debug)]
pub struct RunTestsCmd {
    #[structopt(long = "wasm-opt")]
    pub wasm_opt: bool,
}
