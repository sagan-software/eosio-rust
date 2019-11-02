use eosio::{AccountName, Asset};
use structopt::StructOpt;

/// Transfer tokens from account to account
#[derive(StructOpt, Debug)]
pub struct Transfer {
    /// The account sending tokens
    pub sender: AccountName,
    /// The account receiving tokens
    pub recipient: AccountName,
    /// The amount of tokens to send
    pub amount: Asset,
    /// The memo for the transfer
    pub memo: Option<String>,
    /// The contract which controls the token
    #[structopt(short, long, default_value = "eosio.token")]
    pub contract: AccountName,
    /// Pay ram to open recipient's token balance row
    #[structopt(long)]
    pub pay_ram_to_open: bool,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}
