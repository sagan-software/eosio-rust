// use crate::Cmd;
use clap::{App, Arg, SubCommand};

// pub fn client<'a>(cmd: &clap::ArgMatches<'a>) -> eosio_rpc::Client {
//     let url = cmd.value_of("url").unwrap();
//     eosio_rpc::Client::new(url)
// }

pub fn url<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("url")
        .help("the http/https URL where nodeos is running")
        .long("url")
        .short("u")
        .takes_value(true)
        .default_value("http://127.0.0.1:8888")
}

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("eosrs")
        .version("0.1")
        .about("Command Line Interface to EOSIO Client")
        .author("sagan.software")
        .arg(url())
        .arg(
            Arg::with_name("wallet-url")
                .help("the http/https URL where keosd is running")
                .long("wallet-url")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no-verify")
                .help("don't verify peer certificate when using HTTPS")
                .long("no-verify")
                .short("n"),
        )
    // .subcommand(crate::get::Get::app())
    // .subcommand(crate::gen::Gen::app())
}
