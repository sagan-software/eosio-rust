use eosio::AccountName;
use structopt::StructOpt;

/// Create various items, on and off the blockchain
#[derive(StructOpt, Debug)]
pub enum Create {
    /// Create a new keypair and print the public and private keys
    Key(CreateKey),
    /// Create a new account on the blockchain (assumes system contract does
    /// not restrict RAM usage)
    Account(CreateAccount),
}

/// Create a new keypair and print the public and private keys
#[derive(StructOpt, Debug)]
pub struct CreateKey {
    /// Generate a key using the R1 curve (iPhone), instead of the K1 curve
    /// (Bitcoin)
    #[structopt(long)]
    pub r1: bool,
    /// Name of file to write private/public key output to. (Must be set,
    /// unless "--to-console" is passed)
    #[structopt(short, long, required_unless = "to-console")]
    pub file: Option<String>,
    /// Print private/public keys to console.
    #[structopt(long)]
    pub to_console: bool,
}

/// Create a new account on the blockchain (assumes system contract does not
/// restrict RAM usage)
#[derive(StructOpt, Debug)]
pub struct CreateAccount {
    /// The name of the account creating the new account
    pub creator: AccountName,
    /// The name of the new account
    pub name: AccountName,
    /// The owner public key for the new account
    pub owner_key: String,
    /// The active public key for the new account
    pub active_key: Option<String>,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}
