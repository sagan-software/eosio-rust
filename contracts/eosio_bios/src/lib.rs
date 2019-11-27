use eosio::*;

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

#[eosio::action]
pub fn newaccount(
    _creator: AccountName,
    _name: AccountName,
    _owner: Ignore<Authority>,
    _active: Ignore<Authority>,
) {
}

#[eosio::action]
pub fn updateauth(
    _account: Ignore<AccountName>,
    _permission: Ignore<PermissionName>,
    _parent: Ignore<AccountName>,
    _auth: Ignore<Authority>,
) {
}

#[eosio::action]
pub fn deleteauth(
    _account: Ignore<AccountName>,
    _permission: Ignore<PermissionName>,
) {
}

#[eosio::action]
pub fn linkauth(
    _account: Ignore<AccountName>,
    _code: Ignore<AccountName>,
    _type: Ignore<PermissionName>,
    _requirement: Ignore<PermissionName>,
) {
}

#[eosio::action]
pub fn unlinkauth(
    _account: Ignore<AccountName>,
    _code: Ignore<AccountName>,
    _type: Ignore<PermissionName>,
    _requirement: Ignore<PermissionName>,
) {
}

#[eosio::action]
pub fn canceldelay(
    _canceling_auth: Ignore<PermissionLevel>,
    _trx_id: Ignore<Checksum256>,
) {
}

#[eosio::action]
pub fn onerror(_sender_id: Ignore<u128>, _sent_trx: Ignore<Vec<char>>) {}

#[eosio::action]
pub fn setcode(
    _account: AccountName,
    _vmtype: u8,
    _vmversion: u8,
    _code: Vec<char>,
) {
}

#[eosio::action]
pub fn setpriv(account: AccountName, is_priv: bool) {
    require_auth(current_receiver());
    set_privileged(account, is_priv);
}

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

#[eosio::action]
pub fn setglimits(ram: u64, net: u64, cpu: u64) {
    require_auth(current_receiver());
}

#[eosio::action]
pub fn setprods(schedule: Vec<ProducerKey>) {
    require_auth(current_receiver());
    set_proposed_producers(&schedule);
}

#[eosio::action]
pub fn setparams(params: BlockchainParameters) {
    require_auth(current_receiver());
    set_blockchain_parameters(&params).expect("write");
}

#[eosio::action]
pub fn reqauth(from: AccountName) {
    require_auth(from);
}

#[eosio::action]
pub fn setabi(account: AccountName, abi: Vec<u8>) {
    let data = std::str::from_utf8(&abi).expect("couldn't convert abi to str");
    let hash = sha256(data);
    let _self = current_receiver();
    let table = AbiHash::table(_self, _self);
    match table.find(account) {
        Some(cursor) => {
            let mut row = cursor.get().expect("read");
            row.hash = hash;
            cursor.modify(None, &row).expect("write");
        }
        None => {
            let row = AbiHash {
                owner: account,
                hash,
            };
            table.emplace(account, &row).expect("write");
            let a = table.find(account).unwrap().get().unwrap();
            assert_sha256(&a.hash, data);
            assert!(false);
        }
    }
}

#[derive(Default, Table, Read, Write, NumBytes)]
#[eosio(name = "abihash")]
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
    onerror,
    setcode,
    setpriv,
    setalimits,
    setglimits,
    setprods,
    setparams,
    reqauth,
    setabi
);
