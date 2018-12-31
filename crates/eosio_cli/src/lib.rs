mod app;
pub mod gen;
pub mod get;

pub use crate::app::app;

use clap::{App, Arg, ArgMatches, SubCommand};
use futures::future::Future;

pub trait Cmd<'a, 'b> {
    const NAME: &'a str;
    fn app() -> App<'a, 'b>;
    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send>;
    fn client(cmd: &ArgMatches<'a>) -> eosio_rpc::Client {
        let url = cmd.value_of("url").unwrap();
        eosio_rpc::Client::new(url)
    }
}
