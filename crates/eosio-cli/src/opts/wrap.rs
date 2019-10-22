use eosio::AccountName;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Wrap {
    /// Execute a transaction while bypassing authorization checks
    Exec(Exec),
}

/// Execute a transaction while bypassing authorization checks
#[derive(StructOpt, Debug)]
pub struct Exec {
    /// Account executing the transaction and paying for the deferred
    /// transaction RAM
    pub executer: AccountName,
    /// The JSON string or filename defining the transaction to execute
    pub transaction: String,
    /// The account which controls the wrap contract
    #[structopt(short, long, default_value = "eosio.wrap")]
    pub contract: AccountName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}
