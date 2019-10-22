use structopt::StructOpt;

/// Sign a transaction
#[derive(StructOpt, Debug)]
pub struct Sign {
    /// The JSON string or filename defining the transaction to sign
    pub transaction: String,
    /// The private key that will be used to sign the transaction
    #[structopt(short = "k", long)]
    pub private_key: Option<String>,
    /// The chain id that will be used to sign the transaction
    #[structopt(short, long)]
    pub chain_id: Option<String>,
    /// Push transaction after signing
    #[structopt(short, long)]
    pub push_transaction: bool,
}
