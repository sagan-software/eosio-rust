use clap::{App, Arg, ArgMatches, SubCommand};
use eosio::AccountName;
use futures::future::{self, Future};
use hyper::rt;

pub struct Get;

impl<'a, 'b> crate::Cmd<'a, 'b> for Get {
    const NAME: &'a str = "get";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve various items and information from the blockchain")
            .subcommand(Abi::app())
            .subcommand(Account::app())
            .subcommand(Accounts::app())
            .subcommand(Actions::app())
            .subcommand(Block::app())
            .subcommand(Code::app())
            .subcommand(Currency::app())
            .subcommand(Info::app())
            .subcommand(Schedule::app())
            .subcommand(Scope::app())
            .subcommand(Servants::app())
            .subcommand(Table::app())
            .subcommand(Transaction::app())
            .subcommand(TransactionId::app())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        match cmd.subcommand() {
            (Abi::NAME, Some(sub)) => Abi::handle(sub),
            (Account::NAME, Some(sub)) => Account::handle(sub),
            (Accounts::NAME, Some(sub)) => Accounts::handle(sub),
            (Actions::NAME, Some(sub)) => Actions::handle(sub),
            (Block::NAME, Some(sub)) => Block::handle(sub),
            (Code::NAME, Some(sub)) => Code::handle(sub),
            (Currency::NAME, Some(sub)) => Currency::handle(sub),
            (Info::NAME, Some(sub)) => Info::handle(sub),
            (Schedule::NAME, Some(sub)) => Schedule::handle(sub),
            (Scope::NAME, Some(sub)) => Scope::handle(sub),
            (Servants::NAME, Some(sub)) => Servants::handle(sub),
            (Table::NAME, Some(sub)) => Table::handle(sub),
            (Transaction::NAME, Some(sub)) => Transaction::handle(sub),
            (TransactionId::NAME, Some(sub)) => TransactionId::handle(sub),
            _ => Box::new(future::err(format!("{}", cmd.usage()))),
        }
    }
}

const DEFAULT_URL: &str = "http://127.0.0.1:8888";

struct Abi;

impl<'a, 'b> crate::Cmd<'a, 'b> for Abi {
    const NAME: &'a str = "abi";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve the ABI for an account")
            .arg(
                Arg::with_name("name")
                    .help("The number or ID of the block to retrieve")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("file")
                    .long("file")
                    .short("f")
                    .help("The name of the file to save the contract .abi to instead of writing to console")
                    .takes_value(true)
            )
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        let client = crate::app::client(cmd);
        match cmd.value_of("name") {
            Some(name_str) => match AccountName::from_str(name_str) {
                Ok(name) => Box::new(
                    eosio_rpc::chain::get_abi(name)
                        .fetch(&client)
                        .map(|info| format!("{:#?}", info))
                        .map_err(|err| format!("{:#?}", err)),
                ),
                Err(err) => Box::new(future::err(format!("{:#?}", err))),
            },
            None => Box::new(future::err(format!("{}", cmd.usage()))),
        }
    }
}

struct Account;

impl<'a, 'b> crate::Cmd<'a, 'b> for Account {
    const NAME: &'a str = "account";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve an account from the blockchain")
            .arg(
                Arg::with_name("name")
                    .help("The name of the account to retrieve")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("symbol")
                    .help("The expected core symbol of the chain you are querying")
                    .index(2),
            )
            .arg(
                Arg::with_name("json")
                    .long("json")
                    .short("j")
                    .help("Output in JSON format"),
            )
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        let client = crate::app::client(cmd);
        match cmd.value_of("name") {
            Some(name_str) => match AccountName::from_str(name_str) {
                Ok(name) => Box::new(
                    eosio_rpc::chain::get_account(name)
                        .fetch(&client)
                        .map(|info| format!("{:#?}", info))
                        .map_err(|err| format!("{:#?}", err)),
                ),
                Err(err) => Box::new(future::err(format!("{:#?}", err))),
            },
            None => Box::new(future::err(format!("{}", cmd.usage()))),
        }
    }
}

struct Accounts;

impl<'a, 'b> crate::Cmd<'a, 'b> for Accounts {
    const NAME: &'a str = "accounts";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve accounts associated with a public key")
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        Box::new(future::err("TODO".to_string()))
    }
}

struct Actions;

impl<'a, 'b> crate::Cmd<'a, 'b> for Actions {
    const NAME: &'a str = "actions";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
                    .about("Retrieve all actions with a specific account name referenced in authorization or receiver")
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        Box::new(future::err("TODO".to_string()))
    }
}

struct Block;

impl<'a, 'b> crate::Cmd<'a, 'b> for Block {
    const NAME: &'a str = "block";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve a full block from the blockchain")
            .arg(
                Arg::with_name("block-num-or-id")
                    .help("The number or ID of the block to retrieve")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("header-state")
                    .long("header-state")
                    .help("Get block header state from fork database instead"),
            )
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        let client = crate::app::client(cmd);
        match cmd.value_of("block-num-or-id") {
            Some(block_num_or_id) => Box::new(
                eosio_rpc::chain::get_block(block_num_or_id)
                    .fetch(&client)
                    .map(|info| format!("{:#?}", info))
                    .map_err(|err| format!("{:#?}", err)),
            ),
            None => Box::new(future::err(format!("{}", cmd.usage()))),
        }
    }
}

struct Code;

impl<'a, 'b> crate::Cmd<'a, 'b> for Code {
    const NAME: &'a str = "code";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve the code and ABI for an account")
            .arg(
                Arg::with_name("name")
                    .help("The name of the account whose code should be retrieved")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("code")
                    .long("code")
                    .short("c")
                    .takes_value(true)
                    .help("The name of the file to save the contract .wast/wasm to"),
            )
            .arg(
                Arg::with_name("abi")
                    .long("abi")
                    .short("a")
                    .takes_value(true)
                    .help("The name of the file to save the contract .abi to"),
            )
            .arg(
                Arg::with_name("wasm")
                    .long("wasm")
                    .help("Save contract as wasm"),
            )
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        Box::new(future::err("TODO".to_string()))
    }
}

struct Currency;

impl<'a, 'b> crate::Cmd<'a, 'b> for Currency {
    const NAME: &'a str = "currency";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve information related to standard currencies")
            .subcommand(CurrencyBalance::app())
            .subcommand(CurrencyStats::app())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        match cmd.subcommand() {
            (CurrencyBalance::NAME, Some(sub)) => CurrencyBalance::handle(sub),
            (CurrencyStats::NAME, Some(sub)) => CurrencyStats::handle(sub),
            _ => Box::new(future::err(format!("{}", cmd.usage()))),
        }
    }
}

struct CurrencyBalance;

impl<'a, 'b> crate::Cmd<'a, 'b> for CurrencyBalance {
    const NAME: &'a str = "balance";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve the balance of an account for a given currency")
            .arg(
                Arg::with_name("contract")
                    .help("The contract that operates the currency")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("account")
                    .help("The account to query balances for")
                    .required(true)
                    .index(2),
            )
            .arg(
                Arg::with_name("symbol")
                    .help(
                        "The symbol for the currency if the contract operates multiple currencies",
                    )
                    .index(3),
            )
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        let client = crate::app::client(cmd);
        match (cmd.value_of("contract"), cmd.value_of("account")) {
            (Some(contract), Some(account)) => match (
                eosio::sys::string_to_name(contract),
                eosio::sys::string_to_name(account),
            ) {
                (Ok(contract), Ok(account)) => Box::new(
                    eosio_rpc::chain::get_currency_balance(
                        contract,
                        account,
                        cmd.value_of("symbol"),
                    )
                    .fetch(&client)
                    .map(|info| format!("{:#?}", info))
                    .map_err(|err| format!("{:#?}", err)),
                ),
                _ => Box::new(future::err(format!("{}", cmd.usage()))),
            },
            _ => Box::new(future::err(format!("{}", cmd.usage()))),
        }
    }
}

struct CurrencyStats;

impl<'a, 'b> crate::Cmd<'a, 'b> for CurrencyStats {
    const NAME: &'a str = "stats";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve the stats for a given currency")
            .arg(
                Arg::with_name("contract")
                    .help("The contract that operates the currency")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("symbol")
                    .help(
                        "The symbol for the currency if the contract operates multiple currencies",
                    )
                    .index(2),
            )
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        let client = crate::app::client(cmd);
        match cmd.value_of("contract") {
            Some(contract) => match eosio::sys::string_to_name(contract) {
                Ok(contract) => Box::new(
                    eosio_rpc::chain::get_currency_stats(contract, cmd.value_of("symbol"))
                        .fetch(&client)
                        .map(|info| format!("{:#?}", info))
                        .map_err(|err| format!("{:#?}", err)),
                ),
                _ => Box::new(future::err(format!("{}", cmd.usage()))),
            },
            _ => Box::new(future::err(format!("{}", cmd.usage()))),
        }
    }
}

struct Info;

impl<'a, 'b> crate::Cmd<'a, 'b> for Info {
    const NAME: &'a str = "info";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Get current blockchain information")
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        let client = crate::app::client(cmd);
        Box::new(
            eosio_rpc::chain::get_info()
                .fetch(&client)
                .map(|info| format!("{:#?}", info))
                .map_err(|err| format!("{:#?}", err)),
        )
    }
}

struct Schedule;

impl<'a, 'b> crate::Cmd<'a, 'b> for Schedule {
    const NAME: &'a str = "schedule";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve the producer schedule")
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        Box::new(future::err("TODO".to_string()))
    }
}

struct Scope;

impl<'a, 'b> crate::Cmd<'a, 'b> for Scope {
    const NAME: &'a str = "scope";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve a list of scopes and tables owned by a contract")
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        Box::new(future::err("TODO".to_string()))
    }
}

struct Servants;

impl<'a, 'b> crate::Cmd<'a, 'b> for Servants {
    const NAME: &'a str = "servants";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve accounts which are servants of a given account")
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        Box::new(future::err("TODO".to_string()))
    }
}

struct Table;

impl<'a, 'b> crate::Cmd<'a, 'b> for Table {
    const NAME: &'a str = "table";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve the contents of a database table")
            .arg(Arg::with_name("account")
                .help("The account who owns the table")
                .required(true)
                .index(1)
            )
            .arg(Arg::with_name("scope")
                .help("The scope within the contract in which the table is found")
                .required(true)
                .index(2)
            )
            .arg(Arg::with_name("table")
                .help("The name of the table as specified by the contract abi")
                .required(true)
                .index(3)
            )
            .arg(Arg::with_name("binary")
                .help("Return the value as BINARY rather than using abi to interpret as JSON")
                .long("binary")
                .short("b")
            )
            .arg(Arg::with_name("limit")
                .help("The maximum number of rows to return")
                .long("limit")
                .short("l")
                .takes_value(true)
            )
            .arg(Arg::with_name("lower")
                .help("JSON representation of lower bound value of key, defaults to first")
                .long("lower")
                .short("L")
                .takes_value(true)
            )
            .arg(Arg::with_name("upper")
                .help("JSON representation of upper bound value of key, defaults to last")
                .long("upper")
                .short("U")
                .takes_value(true)
            )
            .arg(Arg::with_name("index")
                .help("Index number, 1 - primary (first), 2 - secondary index (in order defined by multi_index), 3 - third index, etc.\nNumber or name of index can be specified, e.g. 'secondary' or '2'.")
                .long("index")
                .takes_value(true)
            )
            .arg(Arg::with_name("key-type")
                .help("The key type of --index, primary only supports (i64), all others support (i64, i128, i256, float64, float128, ripemd160, sha256).\nSpecial type 'name' indicates an account name.")
                .long("key-type")
                .takes_value(true)
            )
            .arg(Arg::with_name("encode-type")
                .help("The encoding type of key_type (i64, i128, float64, float128) only support decimal encoding e.g. 'dec'i256 - supports both 'dec' and 'hex', ripemd160 and sha256 is 'hex' only")
                .long("encode-type")
                .takes_value(true)
            )
            .arg(Arg::with_name("reverse")
                .help("Iterate in reverse order")
                .long("reverse")
                .short("r")
            )
            .arg(Arg::with_name("show-payer")
                .help("show RAM payer")
                .long("show-payer")
            )
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        let client = crate::app::client(cmd);
        match (
            cmd.value_of("account"),
            cmd.value_of("scope"),
            cmd.value_of("table"),
        ) {
            (Some(account), Some(scope), Some(table)) => match (
                eosio::sys::string_to_name(account),
                eosio::sys::string_to_name(scope),
                eosio::sys::string_to_name(table),
            ) {
                (Ok(account), Ok(scope), Ok(table)) => Box::new(
                    eosio_rpc::chain::get_table_rows(account, scope, table)
                        .fetch::<serde_json::Value>(&client)
                        .map(|info| format!("{:#?}", info))
                        .map_err(|err| format!("{:#?}", err)),
                ),
                _ => Box::new(future::err(format!("{}", cmd.usage()))),
            },
            _ => Box::new(future::err(format!("{}", cmd.usage()))),
        }
    }
}

struct Transaction;

impl<'a, 'b> crate::Cmd<'a, 'b> for Transaction {
    const NAME: &'a str = "transaction";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Retrieve a transaction from the blockchain")
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        Box::new(future::err("TODO".to_string()))
    }
}

struct TransactionId;

impl<'a, 'b> crate::Cmd<'a, 'b> for TransactionId {
    const NAME: &'a str = "transaction_id";

    fn app() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Get transaction id given transaction object")
            .arg(crate::app::url())
    }

    fn handle(cmd: &ArgMatches<'a>) -> Box<Future<Item = String, Error = String> + Send> {
        Box::new(future::err("TODO".to_string()))
    }
}
