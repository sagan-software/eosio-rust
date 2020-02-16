pub mod convert;
pub mod create;
pub mod get;
pub mod multisig;
pub mod net;
pub mod push;
pub mod set;
pub mod sign;
pub mod system;
pub mod transfer;
pub mod version;
pub mod wallet;
pub mod wrap;
use eosio::{BlockNumOrId, PermissionLevel};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "eosrs")]
pub struct Opt {
    /// the http/https URL where nodeos is running
    #[structopt(short, long, default_value = "http://127.0.0.1:8888/")]
    pub url: String,
    /// the http/https URL where keosd is running
    #[structopt(long, default_value = "unix:///root/eosio-wallet/keosd.sock")]
    pub wallet_url: String,
    /// pass specific HTTP header; repeat this option to pass multiple headers
    #[structopt(short = "r", long)]
    pub header: Option<Vec<String>>,
    /// don't verify peer certificate when using HTTPS
    #[structopt(short, long)]
    pub no_verify: bool,
    /// don't automatically launch a keosd if one is not currently running
    #[structopt(long)]
    pub no_auto_keosd: bool,
    /// output verbose actions on error
    #[structopt(short, long)]
    pub verbose: bool,
    /// print HTTP request to STDERR
    #[structopt(long)]
    pub print_request: bool,
    /// print HTTP response to STDERR
    #[structopt(long)]
    pub print_response: bool,
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(StructOpt, Debug)]
pub enum Subcommand {
    /// Pack and unpack transactions
    Convert(self::convert::Convert),
    /// Create various items, on and off the blockchain
    Create(self::create::Create),
    /// Retrieve various items and information from the blockchain
    Get(self::get::Get),
    /// Multisig contract commands
    Multisig(self::multisig::Multisig),
    /// Interact with local p2p network connections
    Net(self::net::Net),
    /// Push arbitrary transactions to the blockchain
    Push(self::push::Push),
    /// Set or update blockchain state
    Set(self::set::Set),
    /// Sign a transaction
    Sign(self::sign::Sign),
    /// Send eosio.system contract aciton to the blockchain
    System(self::system::System),
    /// Transfer tokens from account to account
    Transfer(self::transfer::Transfer),
    /// Retrieve version information
    Version(self::version::Version),
    /// Interact with local wallet
    Wallet(self::wallet::Wallet),
    /// Wrap contract commands
    Wrap(self::wrap::Wrap),
}

#[derive(StructOpt, Debug)]
pub struct TransactionOpts {
    /// set the time in seconds before a transaction expires
    #[structopt(short = "x", long, default_value = "30")]
    pub expiration: u64,
    /// force the transaction to be unique. this will consume extra bandwidth
    /// and remove any protections against accidently issuing the same
    /// transaction multiple times
    #[structopt(short, long)]
    pub force_unique: bool,
    /// Specify if unlocked wallet keys should be used to sign transaction
    #[structopt(short, long)]
    pub skip_sign: bool,
    /// don't broadcast transaction to the network (just print to stdout)
    #[structopt(short, long)]
    pub dont_broadcast: bool,
    /// used in conjunction with --dont-broadcast to get the packed transaction
    #[structopt(long)]
    pub return_packed: bool,
    /// set the reference block num or block id used for TAPOS (Transaction as
    /// Proof-of-Stake)
    #[structopt(short, long)]
    pub ref_block: Option<BlockNumOrId>,
    /// An account and permission level to authorize, as in
    /// 'account@permission'
    #[structopt(short, long = "permission")]
    pub permission_level: Option<PermissionLevel>,
    /// set an upper limit on the milliseconds of cpu usage budget, for the
    /// execution of the transaction
    #[structopt(long, default_value = "0")]
    pub max_cpu_usage_ms: u64,
    /// set an upper limit on the net usage budget, in bytes, for the
    /// transaction
    #[structopt(long, default_value = "0")]
    pub max_net_usage: u64,
    /// set the delay_sec seconds
    #[structopt(long, default_value = "0")]
    pub delay_sec: u64,
}
