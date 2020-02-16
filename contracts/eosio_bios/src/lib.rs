use eosio::*;
use eosio_cdt::*;
use std::marker::PhantomData;

#[derive(Read, Write, NumBytes, Default, Clone)]
pub struct PermissionLevelWeight {
    pub permission: PermissionLevel,
    pub weight: u16,
}

#[derive(Read, Write, NumBytes, Default, Clone)]
pub struct KeyWeight {
    pub key: PublicKey,
    pub weight: u16,
}

#[derive(Read, Write, NumBytes, Default, Clone)]
pub struct WaitWeight {
    pub wait_sec: u32,
    pub weight: u16,
}

#[derive(Read, Write, NumBytes, Default, Clone)]
pub struct Authority {
    pub threshold: u32,
    pub keys: Vec<KeyWeight>,
    pub accounts: Vec<PermissionLevelWeight>,
    pub waits: Vec<WaitWeight>,
}

#[derive(Read, Write, NumBytes, Default, Clone)]
pub struct BlockHeader {
    pub timestamp: u32,
    pub producer: AccountName,
    pub confirmed: u16,
    pub previous: Checksum256,
    pub transaction_mroot: Checksum256,
    pub action_mroot: Checksum256,
    pub schedule_version: u32,
    pub new_producers: Option<ProducerSchedule>,
}

/// New account action
///
/// Called after a new account is created. This code enforces resource-limits
/// rules for new accounts as well as new account naming conventions.
///
///     1. accounts cannot contain '.' symbols which forces all acccounts to be
/// 12        characters long without '.' until a future account auction process
/// is implemented        which prevents name squatting.
///     2. new accounts must stake a minimal number of tokens (as set in system
/// parameters)        therefore, this method will execute an inline buyram from
/// receiver for newacnt in        an amount equal to the current new account
/// creation fee.
#[eosio::action]
pub fn newaccount(
    _creator: AccountName,
    _name: AccountName,
    _owner: PhantomData<Authority>,
    _active: PhantomData<Authority>,
) {
}

/// Update authorization action.
///
/// Updates pemission for an account.
///
/// - `account` - the account for which the permission is updated,
/// - `pemission` - the permission name which is updated,
/// - `parent` - the parent of the permission which is updated,
/// - `auth` - the json describing the permission authorization.
#[eosio::action]
pub fn updateauth(
    _account: PhantomData<AccountName>,
    _permission: PhantomData<PermissionName>,
    _parent: PhantomData<AccountName>,
    _auth: PhantomData<Authority>,
) {
}

/// Delete authorization action.
///
/// Deletes the authorization for an account's permision.
///
/// - `account` - the account for which the permission authorization is deleted,
/// - `permission` - the permission name been deleted.
#[eosio::action]
pub fn deleteauth(
    _account: PhantomData<AccountName>,
    _permission: PhantomData<PermissionName>,
) {
}

/// Link authorization action.
///
/// Assigns a specific action from a contract to a permission you have created.
/// Five system actions can not be linked `updateauth`, `deleteauth`,
/// `linkauth`, `unlinkauth`, and `canceldelay`. This is useful because when
/// doing authorization checks, the EOSIO based blockchain starts with the
/// action needed to be authorized (and the contract belonging to), and looks up
/// which permission is needed to pass authorization validation. If a link is
/// set, that permission is used for authoraization validation otherwise then
/// active is the default, with the exception of `eosio.any`. `eosio.any` is an
/// implicit permission which exists on every account; you can link actions to
/// `eosio.any` and that will make it so linked actions are accessible to any
/// permissions defined for the account.
///
/// - `account` - the permission's owner to be linked and the payer of the RAM
///   needed to store this link,
/// - `code` - the owner of the action to be linked,
/// - `type` - the action to be linked,
/// - `requirement` - the permission to be linked.
#[eosio::action]
pub fn linkauth(
    _account: PhantomData<AccountName>,
    _code: PhantomData<AccountName>,
    _type: PhantomData<PermissionName>,
    _requirement: PhantomData<PermissionName>,
) {
}

/// Unlink authorization action.
///
/// It's doing the reverse of linkauth action, by unlinking the given action.
///
/// - `account` - the owner of the permission to be unlinked and the receiver of
///   the freed RAM,
/// - `code` - the owner of the action to be unlinked,
/// - `type` - the action to be unlinked.
#[eosio::action]
pub fn unlinkauth(
    _account: PhantomData<AccountName>,
    _code: PhantomData<AccountName>,
    _type: PhantomData<PermissionName>,
    _requirement: PhantomData<PermissionName>,
) {
}

/// Cancel delay action.
///
/// Cancels a deferred transaction.
///
/// - `canceling_auth` - the permission that authorizes this action,
/// - `trx_id` - the deferred transaction id to be cancelled.
#[eosio::action]
pub fn canceldelay(
    _canceling_auth: PhantomData<PermissionLevel>,
    _trx_id: PhantomData<Checksum256>,
) {
}

/// Set code action.
///
/// Sets the contract code for an account.
///
/// - `account` - the account for which to set the contract code.
/// - `vmtype` - reserved, set it to zero.
/// - `vmversion` - reserved, set it to zero.
/// - `code` - the code content to be set, in the form of a blob binary..
#[eosio::action]
pub fn setcode(
    _account: AccountName,
    _vmtype: u8,
    _vmversion: u8,
    _code: Vec<char>,
) {
}

/// Set abi for contract.
///
/// Set the abi for contract identified by `account` name. Creates an entry in
/// the abi_hash_table index, with `account` name as key, if it is not already
/// present and sets its value with the abi hash. Otherwise it is updating the
/// current abi hash value for the existing `account` key.
///
/// - `account` - the name of the account to set the abi for
/// - `abi`     - the abi hash represented as a vector of characters
#[eosio::action]
pub fn setabi(account: AccountName, abi: Vec<u8>) {
    let hash = sha256(abi);
    let this = current_receiver();
    let table = AbiHash::table(this, this);
    match table.find(account) {
        Some(cursor) => {
            let mut row = cursor.get().expect("read");
            row.hash = hash;
            cursor.modify(Payer::Same, row).expect("write");
        }
        None => {
            let row = AbiHash {
                owner: account,
                hash,
            };
            table.emplace(account, row).expect("write");
        }
    }
}

/// On error action.
///
/// Notification of this action is delivered to the sender of a deferred
/// transaction when an objective error occurs while executing the deferred
/// transaction. This action is not meant to be called directly.
///
/// - `sender_id` - the id for the deferred transaction chosen by the sender,
/// - `sent_trx` - the deferred transaction that failed.
#[eosio::action]
pub fn onerror(
    _sender_id: PhantomData<u128>,
    _sent_trx: PhantomData<Vec<char>>,
) {
    check(false, "the onerror action cannot be called directly");
}

/// Set privilege status for an account.
///
/// Allows to set privilege status for an account (turn it on/off).
///
/// - `account` - the account to set the privileged status for.
/// - `is_priv` - 0 for false, > 0 for true.
#[eosio::action]
pub fn setpriv(account: AccountName, is_priv: bool) {
    require_auth(current_receiver());
    set_privileged(account, is_priv);
}

/// Set the resource limits of an account
///
/// - `account` - name of the account whose resource limit to be set
/// - `ram_bytes` - ram limit in absolute bytes
/// - `net_weight` - fractionally proportionate net limit of available resources
///   based on (weight / total_weight_of_all_accounts)
/// - `cpu_weight` - fractionally proportionate cpu limit of available resources
///   based on (weight / total_weight_of_all_accounts)
#[eosio::action]
pub fn setalimits(
    account: AccountName,
    ram_bytes: i64,
    net_weight: i64,
    cpu_weight: i64,
) {
    require_auth(current_receiver());
    set_resource_limits(account, ram_bytes, net_weight, cpu_weight);
}

/// Set a new list of active producers, that is, a new producers' schedule.
///
/// Set a new list of active producers, by proposing a schedule change, once the
/// block that contains the proposal becomes irreversible, the schedule is
/// promoted to "pending" automatically. Once the block that promotes the
/// schedule is irreversible, the schedule will become "active".
///
/// - `schedule` - New list of active producers to set
#[eosio::action]
pub fn setprods(schedule: Vec<ProducerKey>) {
    require_auth(current_receiver());
    let _ = set_proposed_producers(schedule);
}

/// Set the blockchain parameters. By tuning these parameters, various degrees
/// of customization can be achieved.
///
/// - `params` - New blockchain parameters to set
#[eosio::action]
pub fn setparams(params: BlockchainParameters) {
    require_auth(current_receiver());
    set_blockchain_parameters(params).expect("write");
}

/// Check if an account has authorization to access current action.
///
/// Checks if the account name `from` passed in as param has authorization to
/// access current action, that is, if it is listed in the action’s allowed
/// permissions vector.
///
/// - `from` - the account name to authorize
#[eosio::action]
pub fn reqauth(from: AccountName) {
    require_auth(from);
}

/// Activates a protocol feature.
///
/// - `feature_digest` - hash of the protocol feature to activate.
#[eosio::action]
pub fn activate(feature_digest: Checksum256) {
    require_auth(current_receiver());
    preactivate_feature(feature_digest);
}

/// Asserts that a protocol feature has been activated
///
/// - `feature_digest` - hash of the protocol feature to check for activation.
#[eosio::action]
pub fn reqactivated(feature_digest: Checksum256) {
    check(
        is_feature_activated(feature_digest),
        "protocol feature is not activated",
    );
}

#[derive(Default, Table, Read, Write, NumBytes)]
#[eosio(table_name = "abihash")]
pub struct AbiHash {
    #[eosio(primary_key)]
    pub owner: AccountName,
    pub hash: Checksum256,
}

eosio::abi!(
    newaccount,
    updateauth,
    deleteauth,
    linkauth,
    unlinkauth,
    canceldelay,
    setcode,
    setabi,
    onerror,
    setpriv,
    setalimits,
    setprods,
    setparams,
    reqauth,
    activate,
    reqactivated
);
