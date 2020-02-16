use eosio::{AccountName, Asset, PermissionName};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum System {
    /// Create a new account on the blockchain with initial resources
    Newaccount(Newaccount),
    /// Register a new producer
    Regproducer(Regproducer),
    /// Unregister an existing producer
    Unregprod(Unregprod),
    /// Vote for a producer
    Voteproducer(Voteproducer),
    /// List producers
    Listproducers(Listproducers),
    /// Delegate bandwidth
    Delegatebw(Delegatebw),
    /// Undelegate bandwidth
    Undelegatebw(Undelegatebw),
    /// List delegated bandwidth
    Listbw(Listbw),
    /// Name bidding
    Bidname(Bidname),
    /// Get bidname info
    Bidnameinfo(Bidnameinfo),
    /// Buy RAM
    Buyram(Buyram),
    /// Sell RAM
    Sellram(Sellram),
    /// Claim producer rewards
    Claimrewards(Claimrewards),
    /// Register an account as a proxy (for voting)
    Regproxy(Regproxy),
    /// Unregister an account as a proxy (for voting)
    Unregproxy(Unregproxy),
    /// Cancel a delayed transaction
    Canceldelay(Canceldelay),
}

/// Create a new account on the blockchain with initial resources
#[derive(StructOpt, Debug)]
pub struct Newaccount {
    /// The name of the account creating the new account
    pub creator: AccountName,
    /// The name of the new account
    pub name: AccountName,
    /// The owner public key for the new account
    pub owner_key: String,
    /// The active public key for the new account
    pub active_key: Option<String>,
    /// The amount of tokens delegated for net bandwidth
    #[structopt(long)]
    pub stake_net: Asset,
    /// The amount of tokens delegated for CPU bandwidth
    #[structopt(long)]
    pub stake_cpu: Asset,
    /// The amount of RAM bytes to purchase for the new account in kibibytes
    /// (KiB)
    #[structopt(long)]
    pub buy_ram_kbytes: Option<u64>,
    /// The amount of RAM bytes to purchase for the new account in bytes
    #[structopt(long)]
    pub buy_ram_bytes: Option<u64>,
    /// The amount of RAM bytes to purchase for the new account in tokens
    #[structopt(long)]
    pub buy_ram: Option<Asset>,
    /// Transfer voting power and right to unstake tokens to receiver
    #[structopt(long)]
    pub transfer: bool,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Register a new producer
#[derive(StructOpt, Debug)]
pub struct Regproducer {
    /// The account to register as a producer
    pub account: AccountName,
    /// The producer's public key
    pub producer_key: String,
    /// url where info about producer can be found
    pub url: Option<String>,
    /// relative location for purpose of nearest neighbor scheduling
    pub location: Option<u64>,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Unregister an existing producer
#[derive(StructOpt, Debug)]
pub struct Unregprod {
    /// The account o unregister as a producer
    pub account: AccountName,
}

/// Vote for a producer
#[derive(StructOpt, Debug)]
pub enum Voteproducer {
    /// Vote your stake through a proxy
    Proxy(Proxy),
    /// Vote for one or more producers
    Prods(Prods),
    /// Add one producer to list of voted producers
    Approve(Approve),
    /// Remove one producer from list of voted producers
    Unapprove(Unapprove),
}

/// Vote your stake through a proxy
#[derive(StructOpt, Debug)]
pub struct Proxy {
    /// The voting account
    pub voter: AccountName,
    /// The proxy account
    pub proxy: AccountName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Vote for one or more producers
#[derive(StructOpt, Debug)]
pub struct Prods {
    /// The voting account
    pub voter: AccountName,
    /// The account(s) to vote for. All options from this position and
    /// following will be treated as the producer list.
    pub producers: Vec<AccountName>,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Add one producer to list of voted producers
#[derive(StructOpt, Debug)]
pub struct Approve {
    /// The voting account
    pub voter: AccountName,
    /// The account to vote for
    pub producer: AccountName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Remove one producer from list of voted producers
#[derive(StructOpt, Debug)]
pub struct Unapprove {
    /// The voting account
    pub voter: AccountName,
    /// The account to remove from voted producers
    pub producer: AccountName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// List producers
#[derive(StructOpt, Debug)]
pub struct Listproducers {
    /// Output in JSON format
    #[structopt(short, long)]
    pub json: bool,
    /// The maximum number of rows to return
    #[structopt(short, long)]
    pub limit: Option<u64>,
    /// lower bound value of key, defaults to first
    #[structopt(short = "L", long)]
    pub lower: Option<String>,
}

/// Delegate bandwidth
#[derive(StructOpt, Debug)]
pub struct Delegatebw {
    /// The account to delegate bandwidth from
    pub from: AccountName,
    /// The account to receive the delegated bandwidth
    pub receiver: AccountName,
    /// The amount of tokens to stake for network bandwidth
    pub stake_net_quantity: Asset,
    /// The amount of tokens to stake for CPU bandwidth
    pub stake_cpu_quantity: Asset,
    /// The amount of tokens to buyram
    #[structopt(long)]
    pub buyram: Option<Asset>,
    /// The amount of RAM to buy in number of bytes
    #[structopt(long)]
    pub buy_ram_bytes: Option<u64>,
    /// Transfer voting power and right to unstake tokens to receiver
    #[structopt(long)]
    pub transfer: bool,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Undelegate bandwidth
#[derive(StructOpt, Debug)]
pub struct Undelegatebw {
    /// The account to delegate bandwidth from
    pub from: AccountName,
    /// The account to receive the delegated bandwidth
    pub receiver: AccountName,
    /// The amount of tokens to undelegate for network bandwidth
    pub unstake_net_quantity: Asset,
    /// The amount of tokens to undelegate for CPU bandwidth
    pub unstake_cpu_quantity: Asset,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// List delegated bandwidth
#[derive(StructOpt, Debug)]
pub struct Listbw {
    /// The account delegated bandwidth
    pub account: AccountName,
    /// Output in JSON format
    #[structopt(short, long)]
    pub json: bool,
}

/// Name bidding
#[derive(StructOpt, Debug)]
pub struct Bidname {
    /// The bidding account
    pub bidder: AccountName,
    /// The bidding name
    pub newname: AccountName,
    /// The amount of tokens to bid
    pub bid: Asset,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Get bidname info
#[derive(StructOpt, Debug)]
pub struct Bidnameinfo {
    /// The bidding name
    pub newname: AccountName,
    /// Output in JSON format
    #[structopt(short, long)]
    pub json: bool,
}

/// Buy RAM
#[derive(StructOpt, Debug)]
pub struct Buyram {
    /// The account paying for RAM
    pub payer: AccountName,
    /// The account receiving bought RAM
    pub receiver: AccountName,
    /// The amount of tokens to pay for RAM, or number of bytes/kibibytes of
    /// RAM if --bytes/--kbytes is set
    pub amount: String,
    /// buyram in number of kibibytes (KiB)
    #[structopt(short, long)]
    pub kbytes: bool,
    /// buyram in number of bytes
    #[structopt(short, long)]
    pub bytes: bool,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Sell RAM
#[derive(StructOpt, Debug)]
pub struct Sellram {
    /// The account to receive tokens for sold RAM
    pub account: AccountName,
    /// Number of RAM bytes to sell
    pub bytes: u64,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Claim producer rewards
#[derive(StructOpt, Debug)]
pub struct Claimrewards {
    /// The account to claim rewards for
    pub owner: AccountName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Register an account as a proxy (for voting)
#[derive(StructOpt, Debug)]
pub struct Regproxy {
    /// The proxy account to register
    pub proxy: AccountName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Unregister an account as a proxy (for voting)
#[derive(StructOpt, Debug)]
pub struct Unregproxy {
    /// The proxy account to unregister
    pub proxy: AccountName,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}

/// Cancel a delayed transaction
#[derive(StructOpt, Debug)]
pub struct Canceldelay {
    /// Account from authorization on the original delayed transaction
    pub canceling_account: AccountName,
    /// Permission from authorization on the original delayed transaction
    pub canceling_permission: PermissionName,
    /// The transaction id of the original delayed transaction
    pub trx_id: String,
    #[structopt(flatten)]
    pub transaction_opts: super::TransactionOpts,
}
