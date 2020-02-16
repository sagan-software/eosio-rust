use eosio::*;
use eosio_cdt::*;
use std::marker::PhantomData;

#[eosio::table("proposal")]
pub struct Proposal {
    #[eosio(primary_key)]
    pub proposal_name: Name,
    pub packed_transaction: Vec<u8>,
}

#[eosio::table("approvals")]
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
    /// requested approval doesn't need to cointain time, but we want requested
    /// approval to be of exact the same size ad provided approval, in this
    /// case approve/unapprove doesn't change serialized data size. So, we
    /// use the same type.
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

/// Create proposal
///
/// Creates a proposal containing one transaction.
/// Allows an account `proposer` to make a proposal `proposal_name` which has
/// `requested` permission levels expected to approve the proposal, and if
/// approved by all expected permission levels then `trx` transaction can we
/// executed by this proposal. The `proposer` account is authorized and the
/// `trx` transaction is verified if it was authorized by the provided keys and
/// permissions, and if the proposal name doesn’t already exist; if all
/// validations pass the `proposal_name` and `trx` trasanction are saved in the
/// proposals table and the `requested` permission levels to the approvals table
/// (for the `proposer` context). Storage changes are billed to `proposer`.
///
/// - `proposer` - The account proposing a transaction
/// - `proposal_name` - The name of the proposal (should be unique for proposer)
/// - `requested` - Permission levels expected to approve the proposal
/// - `trx` - Proposed transaction
///
/// [Reference implementation](https://github.com/EOSIO/eosio.contracts/blob/8f05770098794c040faf7b98cd966105b6c1ccf1/contracts/eosio.msig/src/eosio.msig.cpp#L9-L57)
#[eosio::action]
pub fn propose(
    _proposer: PhantomData<AccountName>,
    _proposal_name: PhantomData<Name>,
    _requested: PhantomData<Vec<PermissionLevel>>,
    _trx: PhantomData<Transaction>,
) {
    let mut ds = current_data_stream();
    let proposer: AccountName = ds.read().expect("read");
    let proposal_name: Name = ds.read().expect("read");
    let requested: Vec<PermissionLevel> = ds.read().expect("read");
    let packed_transaction = ds.as_remaining_bytes().expect("read");
    let trx_header =
        TransactionHeader::unpack(&packed_transaction).expect("read");

    require_auth(proposer);
    assert!(
        trx_header.expiration >= current_time_point_sec(),
        "transaction expired"
    );

    let this = current_receiver();
    let proposals = Proposal::table(this, proposer);
    assert!(
        proposals.find(proposal_name).is_none(),
        "proposal with the same name exists"
    );

    let has_trx_auth =
        has_transaction_authority_bytes(&packed_transaction, &[], &requested)
            .expect("write");
    assert!(has_trx_auth, "transaction authorization failed");

    let proposal = Proposal {
        proposal_name,
        packed_transaction: packed_transaction.to_vec(),
    };
    proposals.emplace(proposer, proposal).expect("write");

    let approvals = ApprovalsInfo::table(this, proposer);
    let approval = ApprovalsInfo {
        version: 1,
        proposal_name,
        requested_approvals: requested
            .into_iter()
            .map(|level| Approval {
                level,
                time: TimePoint::default(),
            })
            .collect(),
        provided_approvals: Vec::new(),
    };
    approvals.emplace(proposer, approval).expect("write");
}

/// Approve proposal
///
/// Approves an existing proposal
/// Allows an account, the owner of `level` permission, to approve a proposal
/// `proposal_name` proposed by `proposer`. If the proposal's requested approval
/// list contains the `level` permission then the `level` permission is moved
/// from internal `requested_approvals` list to internal `provided_approvals`
/// list of the proposal, thus persisting the approval for the `proposal_name`
/// proposal. Storage changes are billed to `proposer`.
///
/// - `proposer` - The account proposing a transaction
/// - `proposal_name` - The name of the proposal (should be unique for proposer)
/// - `level` - Permission level approving the transaction
/// - `proposal_hash` - Transaction's checksum
///
/// [Reference implementation](https://github.com/EOSIO/eosio.contracts/blob/8f05770098794c040faf7b98cd966105b6c1ccf1/contracts/eosio.msig/src/eosio.msig.cpp#L59-L92)
#[eosio::action]
pub fn approve(
    proposer: AccountName,
    proposal_name: Name,
    level: PermissionLevel,
    proposal_hash: BinaryExtension<Checksum256>,
) {
    require_level(level);
    let this = current_receiver();

    if let Some(proposal_hash) = proposal_hash.as_value() {
        let proposals = Proposal::table(this, proposer);
        let proposal = proposals
            .find(proposal_name)
            .expect("proposal not found")
            .get()
            .expect("read");
        assert_sha256(proposal_hash, proposal.packed_transaction);
    }

    let approvals = ApprovalsInfo::table(this, proposer);

    if let Some(cursor) = approvals.find(proposal_name) {
        let mut approval = cursor.get().expect("read");
        let (mut provided, requested): (Vec<_>, Vec<_>) = approval
            .requested_approvals
            .into_iter()
            .partition(|a| a.level == level);
        let mut provided = provided
            .pop()
            .expect("approval is not on the list of requested approvals");
        provided.time = current_time_point();
        approval.provided_approvals.push(provided);
        approval.requested_approvals = requested;
        cursor.modify(Payer::Same, approval).expect("write");
    } else {
        let old_approvals = OldApprovalsInfo::table(this, proposer);
        let cursor = old_approvals
            .find(proposal_name)
            .expect("proposal not found");
        let mut old_approval = cursor.get().expect("read");
        let (mut provided, requested): (Vec<_>, Vec<_>) = old_approval
            .requested_approvals
            .into_iter()
            .partition(|a| a == &level);
        let provided = provided
            .pop()
            .expect("approval is not on the list of requested approvals");
        old_approval.provided_approvals.push(provided);
        old_approval.requested_approvals = requested;
        cursor.modify(Payer::Same, old_approval).expect("write");
    }
}

/// Revoke proposal
///
/// Revokes an existing proposal
/// This action is the reverse of the `approve` action: if all validations pass
/// the `level` permission is erased from internal `provided_approvals` and
/// added to the internal `requested_approvals` list, and thus un-approve or
/// revoke the proposal.
///
/// - `proposer` - The account proposing a transaction
/// - `proposal_name` - The name of the proposal (should be an existing
///   proposal)
/// - `level` - Permission level revoking approval for proposal
///
/// [Reference implementation](https://github.com/EOSIO/eosio.contracts/blob/8f05770098794c040faf7b98cd966105b6c1ccf1/contracts/eosio.msig/src/eosio.msig.cpp#L94-L116)
#[eosio::action]
pub fn unapprove(
    proposer: AccountName,
    proposal_name: Name,
    level: PermissionLevel,
) {
    require_level(level);
    let this = current_receiver();

    let approvals = ApprovalsInfo::table(this, proposer);
    if let Some(cursor) = approvals.find(proposal_name) {
        let mut approval = cursor.get().expect("read");
        let (mut requested, provided): (Vec<_>, Vec<_>) = approval
            .provided_approvals
            .into_iter()
            .partition(|a| a.level == level);
        let requested =
            requested.pop().expect("no approval previously granted");
        approval.requested_approvals.push(requested);
        approval.provided_approvals = provided;
        cursor.modify(Payer::Same, approval).expect("write");
    } else {
        let old_approvals = OldApprovalsInfo::table(this, proposer);
        let cursor = old_approvals
            .find(proposal_name)
            .expect("proposal not found");
        let mut old_approval = cursor.get().expect("read");
        let (mut requested, provided): (Vec<_>, Vec<_>) = old_approval
            .provided_approvals
            .into_iter()
            .partition(|a| a == &level);
        let requested =
            requested.pop().expect("no approval previously granted");
        old_approval.requested_approvals.push(requested);
        old_approval.provided_approvals = provided;
        cursor.modify(Payer::Same, old_approval).expect("write");
    }
}

/// Cancel proposal
///
/// Cancels an existing proposal
///
/// - `proposer` - The account proposing a transaction
/// - `proposal_name` - The name of the proposal (should be an existing
///   proposal)
/// - `canceler` - The account cancelling the proposal (only the proposer can
///   cancel an unexpired transaction, and the canceler has to be different than
///   the proposer)
///
/// Allows the `canceler` account to cancel the `proposal_name` proposal,
/// created by a `proposer`, only after time has expired on the proposed
/// transaction. It removes corresponding entries from internal proptable and
/// from approval (or old approvals) tables as well.
///
/// [Reference implementation](https://github.com/EOSIO/eosio.contracts/blob/8f05770098794c040faf7b98cd966105b6c1ccf1/contracts/eosio.msig/src/eosio.msig.cpp#L118-L140)
#[eosio::action]
pub fn cancel(
    proposer: AccountName,
    proposal_name: Name,
    canceler: AccountName,
) {
    require_auth(canceler);

    let this = current_receiver();

    let proposals = Proposal::table(this, proposer);
    let cursor = proposals.find(proposal_name).expect("proposal not found");
    let proposal = cursor.get().expect("read");

    if canceler != proposer {
        let trx_header = TransactionHeader::unpack(proposal.packed_transaction)
            .expect("read");
        assert!(
            trx_header.expiration < current_time_point_sec(),
            "cannot cancel until expiration"
        );
    }

    cursor.erase().expect("read");

    let approvals = ApprovalsInfo::table(this, proposer);
    if let Some(cursor) = approvals.find(proposal_name) {
        cursor.erase().expect("read");
    } else {
        let old_approvals = OldApprovalsInfo::table(this, proposer);
        let cursor = old_approvals
            .find(proposal_name)
            .expect("proposal not found");
        cursor.erase().expect("read");
    }
}

/// Execute proposal
///
/// Allows an `executer` account to execute a proposal.
///
/// Preconditions:
/// - `executer` has authorization,
/// - `proposal_name` is found in the proposals table,
/// - all requested approvals are received,
/// - proposed transaction is not expired,
/// - and approval accounts are not found in invalidations table.
///
/// If all preconditions are met the transaction is executed as a deferred
/// transaction, and the proposal is erased from the proposals table.
///
/// - `proposer` - The account proposing a transaction
/// - `proposal_name` - The name of the proposal (should be an existing
///   proposal)
/// - `executer` - The account executing the transaction
///
/// [Reference implementation](https://github.com/EOSIO/eosio.contracts/blob/8f05770098794c040faf7b98cd966105b6c1ccf1/contracts/eosio.msig/src/eosio.msig.cpp#L142-L189)
#[eosio::action]
pub fn exec(proposer: AccountName, proposal_name: Name, executer: AccountName) {
    require_auth(executer);

    let this = current_receiver();
    let proposals = Proposal::table(this, proposer);
    let proposal_cursor =
        proposals.find(proposal_name).expect("proposal not found");
    let proposal = proposal_cursor.get().expect("read");
    let trx_header =
        TransactionHeader::unpack(&proposal.packed_transaction).expect("read");
    assert!(
        trx_header.expiration >= current_time_point_sec(),
        "transaction expired"
    );

    let mut approval_levels: Vec<PermissionLevel> = Vec::new();
    let approvals = ApprovalsInfo::table(this, proposer);
    let invalidations = Invalidation::table(this, this);
    if let Some(cursor) = approvals.find(proposal_name) {
        let approval = cursor.get().expect("read");
        for p in approval.provided_approvals.into_iter() {
            let is_invalidated = invalidations
                .find(p.level.actor)
                .map(|cursor| {
                    let inv = cursor.get().expect("read");
                    inv.last_invalidation_time > p.time
                })
                .unwrap_or(false);
            if !is_invalidated {
                approval_levels.push(p.level);
            }
        }
        cursor.erase().expect("read");
    } else {
        let old_approvals = OldApprovalsInfo::table(this, proposer);
        let cursor = old_approvals
            .find(proposal_name)
            .expect("proposal not found");
        let approval = cursor.get().expect("read");
        for level in approval.provided_approvals.into_iter() {
            let is_invalidated = invalidations.find(level.actor).is_some();
            if !is_invalidated {
                approval_levels.push(level);
            }
        }
        cursor.erase().expect("read");
    }

    let has_trx_auth = has_transaction_authority_bytes(
        &proposal.packed_transaction,
        &[],
        approval_levels,
    )
    .expect("write");
    assert!(has_trx_auth, "transaction authorization failed");

    let trx_id = TransactionId::from(
        (u128::from(proposer.as_u64()) << 64)
            | u128::from(proposal_name.as_u64()),
    );
    send_deferred_bytes(trx_id, executer, proposal.packed_transaction, true);
    proposal_cursor.erase().expect("read");
}

/// Invalidate proposal
///
/// Allows an `account` to invalidate itself, that is, its name is added to
/// the invalidations table and this table will be cross referenced when exec is
/// performed.
///
/// - `account` - The account invalidating the transaction
///
/// [Reference implementation](https://github.com/EOSIO/eosio.contracts/blob/8f05770098794c040faf7b98cd966105b6c1ccf1/contracts/eosio.msig/src/eosio.msig.cpp#L191-L205)
#[eosio::action]
pub fn invalidate(account: AccountName) {
    require_auth(account);
    let this = current_receiver();
    let invalidations = Invalidation::table(this, this);
    if let Some(cursor) = invalidations.find(account) {
        let mut invalidation = cursor.get().expect("read");
        invalidation.last_invalidation_time = current_time_point();
        cursor.modify(Payer::Same, invalidation).expect("write");
    } else {
        let invalidation = Invalidation {
            account,
            last_invalidation_time: current_time_point(),
        };
        invalidations.emplace(account, invalidation).expect("write");
    }
}

eosio::abi!(propose, approve, unapprove, cancel, exec, invalidate);
