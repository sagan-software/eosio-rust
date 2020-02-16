use eosio::{AccountName, ActionName};
use structopt::StructOpt;

/// Push arbitrary transactions to the blockchain
#[derive(StructOpt, Debug)]
pub enum Push {
    /// Push a transaction with a single action
    Action(Action),
    /// Push an arbitrary JSON transaction
    Transaction(Transaction),
    /// Push an array of arbitrary JSON transactions
    Transactions(Transactions),
}

/// Push a transaction with a single action
#[derive(StructOpt, Debug)]
pub struct Action {
    /// The account providing the contract to execute
    pub account: AccountName,
    /// A JSON string or filename defining the action to execute on the
    /// contract
    pub action: ActionName,
    /// The arguments to the contract
    pub data: String,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Push an arbitrary JSON transaction
#[derive(StructOpt, Debug)]
pub struct Transaction {
    /// The JSON string or filename defining the transaction to push
    pub transaction: String,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Push an array of arbitrary JSON transactions
#[derive(StructOpt, Debug)]
pub struct Transactions {
    /// The JSON string or filename defining the transactions to push
    pub transactions: String,
}
