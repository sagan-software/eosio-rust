use clap::{App, Arg, ArgMatches, SubCommand};
use eosio::AccountName;
use futures::future::{self, Future};

pub struct Gen;

impl<'a, 'b> crate::Cmd<'a, 'b> for Gen {
    const NAME: &'a str = "gen";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Generate code")
            .subcommand(Abi::app())
            .subcommand(Rust::app())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        match cmd.subcommand() {
            (Abi::NAME, Some(sub)) => Abi::handle(sub),
            (Rust::NAME, Some(sub)) => Rust::handle(sub),
            _ => Box::new(future::err(cmd.usage().to_string())),
        }
    }
}

struct Abi;

impl<'a, 'b> crate::Cmd<'a, 'b> for Abi {
    const NAME: &'a str = "abi";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME).about("Generate an ABI JSON file")
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        Box::new(future::ok("ABI!".to_string()))
    }
}

struct Rust;

impl<'a, 'b> crate::Cmd<'a, 'b> for Rust {
    const NAME: &'a str = "rust";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Generate Rust code from an ABI")
            .arg(crate::app::url().requires("name"))
            .arg(
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .takes_value(true)
                    .help("Contract account name to fetch the ABI for")
                    .required_unless("file"),
            )
            .arg(
                Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .takes_value(true)
                    .help("File path to an ABI JSON file to generate code for")
                    .required_unless("name"),
            )
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        let url = cmd
            .value_of("url")
            .unwrap_or_else(|| "http://127.0.0.1:8888");
        let client = eosio_rpc::Client::new(url);
        match cmd.value_of("name") {
            Some(name_str) => match AccountName::from_str(name_str) {
                Ok(name) => Box::new(
                    eosio_rpc::chain::get_abi(name)
                        .fetch(&client)
                        .map(|res| format!("{}", res.abi.to_code().to_string()))
                        .map_err(|err| format!("{:#?}", err)),
                ),
                Err(err) => Box::new(future::err(format!("{:#?}", err))),
            },
            None => Box::new(future::err(format!("{}", cmd.usage()))),
        }
    }
}
