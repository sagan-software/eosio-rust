extern crate eosio_cli;

use eosio_cli::Cmd;
use futures::future::{self, Future};

fn main() {
    let app = eosio_cli::app().get_matches();

    let fut = match app.subcommand() {
        ("get", Some(sub1)) => eosio_cli::get::Get::handle(sub1),
        ("gen", Some(sub1)) => eosio_cli::gen::Gen::handle(sub1),
        _ => Box::new(future::err(format!("{}", app.usage()))),
    };

    hyper::rt::run(
        fut.map(|result| println!("{}", result))
            .map_err(|error| eprintln!("{}", error)),
    );
}
