use eosio::*;

#[eosio::table("proposal")]
pub struct Proposal {
    #[eosio(primary_key)]
    pub proposal_name: Name,
    pub packed_transaction: Vec<char>,
}

#[eosio::table("proposal")]
pub struct OldApprovalsInfo {
    #[eosio(primary_key)]
    pub proposal_name: Name,
    pub requested_approvals: Vec<PermissionLevel>,
    pub provided_approvals: Vec<PermissionLevel>,
}

#[eosio::table("approvals2")]
pub struct ApprovalsInfo {
    pub version: u8,
    #[eosio(primary_key)]
    pub proposal_name: Name,
    pub requested_approvals: Vec<Approval>,
    pub provided_approvals: Vec<Approval>,
}

#[derive(
    Read, Write, NumBytes, Default, Clone, PartialEq, PartialOrd, Debug,
)]
pub struct Approval {
    pub level: PermissionLevel,
    pub time: TimePoint,
}

#[eosio::table("invals")]
pub struct Invalidation {
    #[eosio(primary_key)]
    pub account: AccountName,
    pub last_invalidation_time: TimePoint,
}

#[eosio::action]
pub fn propose(
    _proposer: Ignore<AccountName>,
    _proposal_name: Ignore<Name>,
    _requested: Ignore<Vec<PermissionLevel>>,
    _trx: Ignore<Transaction>,
) {
    let mut ds = current_data_stream();
    let proposer: AccountName = ds.read().expect("read");
    let proposal_name: Name = ds.read().expect("read");
    let requested: Vec<PermissionLevel> = ds.read().expect("read");
    let packed_transaction = ds.as_bytes();
    let trx_header =
        TransactionHeader::read(&packed_transaction, &mut 0).expect("read");

    require_auth(proposer);
    assert!(
        trx_header.expiration >= current_time_point_sec(),
        "transaction expired"
    );

    let _self = current_receiver();
    let proposals = Proposal::table(_self, proposer);
    assert!(
        proposals.find(proposal_name).is_none(),
        "proposal with the same name exists"
    );
}

#[eosio::action]
pub fn approve(
    proposer: AccountName,
    proposal_name: Name,
    level: PermissionLevel,
    proposal_hash: Checksum256,
) {
}

#[eosio::action]
pub fn unapprove(
    proposer: AccountName,
    proposal_level: Name,
    level: PermissionLevel,
) {
}

#[eosio::action]
pub fn cancel(
    proposer: AccountName,
    proposal_name: Name,
    canceler: AccountName,
) {
}

#[eosio::action]
pub fn exec(proposer: AccountName, proposal_name: Name, executer: AccountName) {
}

#[eosio::action]
pub fn invalidate(account: AccountName) {}

eosio::abi!(propose, approve, unapprove, cancel, exec, invalidate);
