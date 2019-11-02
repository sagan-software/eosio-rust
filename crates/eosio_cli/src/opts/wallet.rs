use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Wallet {
    /// Create a new wallet locally
    Create(Create),
    /// Open an existing wallet
    Open(Open),
    /// Lock wallet
    Lock(Lock),
    /// Lock all unlocked wallets
    LockAll,
    /// Unlock wallet
    Unlock(Unlock),
    /// Import private key into wallet
    Import(Import),
    /// Remove key from wallet
    RemoveKey(RemoveKey),
    /// Create private key within wallet
    CreateKey(CreateKey),
    /// List opened wallets, * = unlocked
    List,
    /// List of public keys from all unlocked wallets.
    Keys,
    /// List of private keys from an unlocked wallet in wif or PVT_R1 format.
    PrivateKeys(PrivateKeys),
    /// Stop keosd (doesn't work with nodeos).
    Stop,
}

/// Create a new wallet locally
#[derive(StructOpt, Debug)]
pub struct Create {
    /// The name of the new wallet
    #[structopt(short, long, default_value = "default")]
    pub name: String,
    /// Name of file to write wallet password output to. (Must be set, unless
    /// "--to-console" is passed)
    #[structopt(short, long, required_unless = "to-console")]
    pub file: Option<String>,
    /// Print password to console.
    #[structopt(long)]
    pub to_console: bool,
}

/// Open an existing wallet
#[derive(StructOpt, Debug)]
pub struct Open {
    /// The name of the wallet to open
    #[structopt(short, long)]
    pub name: String,
}

/// Lock wallet
#[derive(StructOpt, Debug)]
pub struct Lock {
    /// The name of the wallet to lock
    #[structopt(short, long)]
    pub name: String,
}

/// Unlock wallet
#[derive(StructOpt, Debug)]
pub struct Unlock {
    /// The name of the wallet to unlock
    #[structopt(short, long)]
    pub name: String,
    /// The password returned by wallet create
    #[structopt(long)]
    pub password: String,
}

/// Import private key into wallet
#[derive(StructOpt, Debug)]
pub struct Import {
    /// The name of the wallet to import key into
    #[structopt(short, long)]
    pub name: String,
    /// Private key in WIF format to import
    #[structopt(long)]
    pub private_key: String,
}

/// Remove key from wallet
#[derive(StructOpt, Debug)]
pub struct RemoveKey {
    /// Public key in WIF format to remove
    pub key: String,
    /// The name of the wallet to unlock
    #[structopt(short, long)]
    pub name: String,
    /// The password returned by wallet create
    #[structopt(long)]
    pub password: String,
}

/// Create private key within wallet
#[derive(StructOpt, Debug)]
pub struct CreateKey {
    /// Key type to create (K1/R1)
    pub key_type: String,
    /// The name of the wallet to create key into
    #[structopt(short, long, default_value = "default")]
    pub name: String,
}

/// List of private keys from an unlocked wallet in wif or PVT_R1 format.
#[derive(StructOpt, Debug)]
pub struct PrivateKeys {
    /// The name of the wallet to list keys from
    #[structopt(short, long, default_value = "default")]
    pub name: String,
    /// The password returned by wallet create
    #[structopt(long)]
    pub password: String,
}
