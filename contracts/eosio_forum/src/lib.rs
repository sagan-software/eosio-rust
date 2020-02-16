use eosio::*;
use eosio_cdt::*;

#[eosio::action]
pub fn propose(
    proposer: AccountName,
    proposal_name: Name,
    title: String,
    proposal_json: String,
    expires_at: TimePointSec,
) {
}

#[eosio::action]
pub fn expire(proposal_name: Name) {}

#[eosio::action]
pub fn vote(
    voter: AccountName,
    proposal_name: Name,
    vote: u8,
    vote_json: String,
) {
}

#[eosio::action]
pub fn unvote(voter: AccountName, proposal_name: Name) {}

#[eosio::action]
pub fn clnproposal(proposal_name: Name, max_count: u64) {}

#[eosio::action]
pub fn post(
    poster: AccountName,
    post_uuid: String,
    content: String,
    reply_to_poster: AccountName,
    reply_to_poster_uuid: String,
    certify: bool,
    json_metadata: String,
) {
}

#[eosio::action]
pub fn unpost(poster: AccountName, post_uuid: String) {}

#[eosio::action]
pub fn status(account: AccountName, content: String) {}

const FREEZE_PERIOD_IN_SECONDS: u32 = 3 * 24 * 60 * 60;
const SIX_MONTHS_IN_SECONDS: u32 =
    (6.0 * (365.25 / 12.0) * 24.0 * 60.0 * 60.0) as u32;

fn compute_by_proposal_key(proposal_name: Name, voter: AccountName) -> u128 {
    u128::from(proposal_name.as_u64()) << 64 | u128::from(voter.as_u64())
}

fn compute_by_voter_key(proposal_name: Name, voter: AccountName) -> u128 {
    u128::from(voter.as_u64()) << 64 | u128::from(voter.as_u64())
}

#[eosio::table("proposal")]
pub struct ProposalRow {
    #[eosio(primary_key)]
    proposal_name: Name,
    #[eosio(secondary_key)]
    proposer: AccountName,
    title: String,
    proposal_json: String,
    created_at: TimePointSec,
    expires_at: TimePointSec,
}

impl ProposalRow {
    fn is_expired(&self) -> bool {
        current_time_point_sec() >= self.expires_at
    }

    fn can_be_cleaned_up(&self) -> bool {
        current_time_point_sec() > self.expires_at + FREEZE_PERIOD_IN_SECONDS
    }
}

#[derive(Read, Write, NumBytes)]
pub struct VoteRow {
    id: u64,
    proposal_name: Name,
    voter: AccountName,
    vote: u8,
    vote_json: String,
    updated_at: TimePointSec,
}

impl Table for VoteRow {
    type Row = Self;

    const NAME: TableName = TableName::new(n!("vote"));

    fn primary_key(row: &Self::Row) -> u64 {
        row.id
    }

    fn secondary_keys(row: &Self::Row) -> SecondaryKeys {
        (
            compute_by_proposal_key(row.proposal_name, row.voter),
            compute_by_voter_key(row.proposal_name, row.voter),
        )
            .into()
    }
}

#[eosio::table("status")]
pub struct StatusRow {
    #[eosio(primary_key)]
    account: AccountName,
    content: String,
    updated_at: TimePointSec,
}
