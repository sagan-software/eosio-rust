use eosio::{AccountName, ActionName, Name};
use structopt::StructOpt;

/// Multisig contract commands
#[derive(StructOpt, Debug)]
pub enum Multisig {
    /// Propose action
    Propose(Propose),
    /// Propose transaction
    ProposeTrx(ProposeTrx),
    /// Review transaction
    Review(Review),
    /// Approve proposed transaction
    Approve(Approve),
    /// Unapprove proposed transaction
    Unapprove(Unapprove),
    /// Invalidate all multisig approvals of an account
    Invalidate(Invalidate),
    /// Cancel proposed transaction
    Cancel(Cancel),
    /// Execute proposed transaction
    Exec(Exec),
}

/// Propose action
#[derive(StructOpt, Debug)]
pub struct Propose {
    /// proposal name
    pub proposal_name: Name,
    /// The JSON string or filename defining requested permissions
    pub requested_permissions: String,
    /// The JSON string or filenamae defining transaction permissions
    pub trx_permissions: String,
    /// contract to which deferred transaction should be delivered
    pub contract: AccountName,
    /// action of deferred transaction
    pub action: ActionName,
    /// The JSON string or filename defining the action to propose
    pub data: String,
    /// Account proposing the transaction
    pub proposer: Option<AccountName>,
    /// Proposal expiration interval in hours
    pub proposal_expiration: Option<u32>,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Propose transaction
#[derive(StructOpt, Debug)]
pub struct ProposeTrx {
    /// proposal name
    pub proposal_name: AccountName,
    /// The JSON string or filename defining requested permissions
    pub requested_permissions: String,
    /// The JSON string or filename defining the transaction to push
    pub transaction: String,
    /// Account proposing the transaction
    pub proposer: Option<AccountName>,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Review transaction
#[derive(StructOpt, Debug)]
pub struct Review {
    /// proposer name
    pub proposer: AccountName,
    /// proposal name
    pub proposal_name: Name,
    /// Show the status of the approvals requested within the proposal
    #[structopt(long)]
    pub show_approvals: bool,
}

/// Approve proposed transaction
#[derive(StructOpt, Debug)]
pub struct Approve {
    /// proposer name
    pub proposer: AccountName,
    /// proposal name
    pub proposal_name: Name,
    /// The JSON string or filename defining approving permissions
    pub permissions: String,
    /// Hash of proposed transaction (i.e. transaction ID) to optionally
    /// enforce as condition of the approval
    pub proposal_hash: Option<String>,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Unapprove proposed transaction
#[derive(StructOpt, Debug)]
pub struct Unapprove {
    /// proposer name
    pub proposer: AccountName,
    /// proposal name
    pub proposal_name: Name,
    /// The JSON string or filename defining approving permissions
    pub permissions: String,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Invalidate all multisig approvals of an account
#[derive(StructOpt, Debug)]
pub struct Invalidate {
    /// invalidator name
    pub invalidator: AccountName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Cancel proposed transaction
#[derive(StructOpt, Debug)]
pub struct Cancel {
    /// proposer name
    pub proposer: AccountName,
    /// proposal name
    pub proposal_name: Name,
    /// canceler name
    pub canceler: AccountName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Execute proposed transaction
#[derive(StructOpt, Debug)]
pub struct Exec {
    /// proposer name
    pub proposer: AccountName,
    /// proposal name
    pub proposal_name: Name,
    /// account paying for execution
    pub executer: AccountName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}
