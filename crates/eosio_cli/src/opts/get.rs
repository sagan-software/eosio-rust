use eosio::{
    AccountName, BlockNum, BlockNumOrId, ScopeName, SymbolCode, TableName,
};
use structopt::StructOpt;

/// Retrieve various items and information from the blockchain
#[derive(StructOpt, Debug)]
pub enum Get {
    /// Get current blockchain information
    Info,
    /// Retrieve a full block from the blockchain
    Block(GetBlock),
    /// Retrieve an account from the blockchain
    Account(GetAccount),
    /// Retrieve the code and ABI for an account
    Code(GetCode),
    /// Retrieve the ABI for an account
    Abi(GetAbi),
    /// Retrieve the contents of a database table
    Table(GetTable),
    /// Retrieve a list of scopes and tables owned by a contract
    Scope(GetScope),
    /// Retrieve information related to standard currencies
    Currency(GetCurrency),
    /// Retrieve accounts associated with a public key
    Accounts(GetAccounts),
    /// Retrieve accounts which are servants of a given account
    Servants(GetServants),
    /// Retrieve a transaction from the blockchain
    Transaction(GetTransaction),
    /// Retrieve all actions with specific account name referenced in
    /// authorization or receiver
    Actions(GetActions),
    /// Retrieve the producer schedule
    Schedule,
    /// Get transaction id given transaction object
    TransactionId(GetTransactionId),
}

/// Retrieve a full block from the blockchain
#[derive(StructOpt, Debug)]
pub struct GetBlock {
    /// The number or ID of the block to retrieve
    pub block: BlockNumOrId,
    /// Get block header state from fork database instead
    #[structopt(long)]
    pub header_state: bool,
}

/// Retrieve an account from the blockchain
#[derive(StructOpt, Debug)]
pub struct GetAccount {
    /// The name of the account to retrieve
    pub name: AccountName,
    /// The expected core symbol of the chain you are querying
    pub core_symbol: Option<SymbolCode>,
    /// Output in JSON format
    #[structopt(short, long)]
    pub json: bool,
}

/// Retrieve the code and ABI for an account
#[derive(StructOpt, Debug)]
pub struct GetCode {
    /// The name of the account whose code should be retrieved
    pub name: AccountName,
    /// The name of the file to save the contract .wast/wasm to
    #[structopt(short, long)]
    pub code: Option<String>,
    /// The name of the file to save the contract .abi to
    #[structopt(short, long)]
    pub abi: Option<String>,
    /// Save contract as wasm
    #[structopt(long)]
    pub wasm: bool,
}

/// Retrieve the ABI for an account
#[derive(StructOpt, Debug)]
pub struct GetAbi {
    /// The name of the account whose abi should be retrieved
    pub name: AccountName,
    /// The name of the file to save the contract .abi to instead of writing to
    /// console
    #[structopt(short, long)]
    pub file: Option<String>,
}

/// Retrieve the contents of a database table
#[derive(StructOpt, Debug)]
pub struct GetTable {
    /// The account who owns the table
    pub account: AccountName,
    /// The scope within the contract in which the table is found
    pub scope: ScopeName,
    /// The name of the table as specified by the contract abi
    pub table: TableName,
    /// Return the value as BINARY rather than using abi to interpret as JSON
    #[structopt(short, long)]
    pub binary: bool,
    /// The maximum number of rows to return
    #[structopt(short, long)]
    pub limit: Option<u64>,
    /// JSON representation of lower bound value of key, defaults to first
    #[structopt(short = "L", long)]
    pub lower: Option<String>,
    /// JSON representation of upper bound value of key, defaults to last
    #[structopt(short = "U", long)]
    pub upper: Option<String>,
    /// Index number, 1 - primary (first), 2 - secondary index (in order
    /// defined by multi_index), 3 - third index, etc. Number or name of
    /// index can be specified, e.g. 'secondary' or '2'.
    #[structopt(long)]
    pub index: Option<String>,
    /// The key type of --index, primary only supports (i64), all others
    /// support (i64, i128, i256, float64, float128, ripemd160, sha256).
    /// Special type 'name' indicates an account name.
    #[structopt(long)]
    pub key_type: Option<String>,
    /// The encoding type of key_type (i64, i128, float64, float128) only
    /// support decimal encoding e.g. 'dec'i256 - supports both 'dec' and
    /// 'hex', ripemd160 and sha256 is 'hex' only
    #[structopt(long)]
    pub encode_type: Option<String>,
    /// Iterate in reverse order
    #[structopt(short, long)]
    pub reverse: bool,
    /// show RAM payer
    #[structopt(long)]
    pub show_payer: bool,
}

/// Retrieve a list of scopes and tables owned by a contract
#[derive(StructOpt, Debug)]
pub struct GetScope {
    /// The contract who owns the table
    pub contract: AccountName,
    /// The name of the table as filter
    #[structopt(short, long)]
    pub table: Option<TableName>,
    /// The maximum number of rows to return
    #[structopt(short, long)]
    pub limit: Option<u64>,
    /// lower bound of scope
    #[structopt(short = "L", long)]
    pub lower: Option<String>,
    /// upper bound of scope
    #[structopt(short = "U", long)]
    pub upper: Option<String>,
    /// Iterate in reverse order
    #[structopt(short, long)]
    pub reverse: bool,
}

/// Retrieve information related to standard currencies
#[derive(StructOpt, Debug)]
pub enum GetCurrency {
    /// Retrieve the balance of an account for a given currency
    Balance(GetCurrencyBalance),
    /// Retrieve the stats of for a given currency
    Stats(GetCurrencyStats),
}

/// Retrieve the balance of an account for a given currency
#[derive(StructOpt, Debug)]
pub struct GetCurrencyBalance {
    /// The contract that operates the currency
    pub contract: AccountName,
    /// The account to query balances for
    pub account: AccountName,
    /// The symbol for the currency if the contract operates multiple
    /// currencies
    pub symbol: Option<SymbolCode>,
}

/// Retrieve the stats of for a given currency
#[derive(StructOpt, Debug)]
pub struct GetCurrencyStats {
    /// The contract that operates the currency
    pub contract: AccountName,
    /// The symbol for the currency if the contract operates multiple
    /// currencies
    pub symbol: SymbolCode,
}

/// Retrieve accounts associated with a public key
#[derive(StructOpt, Debug)]
pub struct GetAccounts {
    /// The public key to retrieve accounts for
    pub public_key: String,
}

/// Retrieve accounts which are servants of a given account
#[derive(StructOpt, Debug)]
pub struct GetServants {
    /// The name of the controlling account
    pub account: AccountName,
}

/// Retrieve a transaction from the blockchain
#[derive(StructOpt, Debug)]
pub struct GetTransaction {
    /// ID of the transaction to retrieve
    pub id: String,
    /// the block number this transaction may be in
    #[structopt(short, long)]
    pub block_hint: Option<BlockNum>,
}

/// Retrieve all actions with specific account name referenced in authorization
/// or receiver
#[derive(StructOpt, Debug)]
pub struct GetActions {
    /// name of account to query on
    pub account_name: AccountName,
    /// sequence number of action for this account, -1 for last
    pub pos: Option<i64>,
    /// get actions [pos,pos+offset] for positive offset or [pos-offset,pos]
    /// for negative offset
    pub offset: Option<i64>,
    /// print full json
    #[structopt(short, long)]
    pub json: bool,
    /// don't truncate action json
    #[structopt(long)]
    pub full: bool,
    /// pretty print full action json
    #[structopt(long)]
    pub pretty: bool,
    /// print console output generated by action
    #[structopt(long)]
    pub console: bool,
}

/// Get transaction id given transaction object
#[derive(StructOpt, Debug)]
pub struct GetTransactionId {
    /// The JSON string or filename defining the transaction which transaction
    /// id we want to retrieve
    pub transaction: String,
}
