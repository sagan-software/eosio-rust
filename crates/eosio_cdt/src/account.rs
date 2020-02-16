use eosio::{AccountName, PermissionLevel, PermissionName, TimePoint};

/// Get the current receiver of the action.
#[must_use]
#[inline]
pub fn current_receiver() -> AccountName {
    let name = unsafe { eosio_cdt_sys::current_receiver() };
    AccountName::new(name)
}

/// Returns the creation time of an account
#[inline]
pub fn creation_time<A: AsRef<AccountName>>(account: A) -> TimePoint {
    let a = account.as_ref().as_u64();
    let time = unsafe { eosio_cdt_sys::get_account_creation_time(a) };
    TimePoint::from_micros(time)
}

/// Returns the last used time of a permission
#[inline]
pub fn permission_last_used<A, P>(account: A, permission: P) -> TimePoint
where
    A: AsRef<AccountName>,
    P: AsRef<PermissionName>,
{
    let a = account.as_ref().as_u64();
    let p = permission.as_ref().as_u64();
    let time = unsafe { eosio_cdt_sys::get_permission_last_used(a, p) };
    TimePoint::from_micros(time)
}

// TODO: support chains that have more/less than 21 producers
/// Gets the top 21 producers
#[must_use]
#[inline]
pub fn active_producers() -> [Option<AccountName>; 21] {
    let mut producers = [0_u64; 21];
    let producers_ptr: *mut u64 = &mut producers as *mut _ as *mut u64;
    let producers_len: u32 = 168; // 8 * 21;
    unsafe {
        eosio_cdt_sys::get_active_producers(producers_ptr, producers_len)
    };

    let mut options = [None; 21];
    for (index, item) in options.iter_mut().enumerate() {
        *item = match producers.get(index) {
            Some(&producer) => {
                if producer == 0 {
                    None
                } else {
                    Some(AccountName::new(producer))
                }
            }
            None => None,
        }
    }
    options
}

/// Verifies that `name` name has auth.
#[inline]
pub fn has_auth<A: AsRef<AccountName>>(account: A) -> bool {
    let a = account.as_ref().as_u64();
    unsafe { eosio_cdt_sys::has_auth(a) }
}

/// Verifies that `name` is an account.
#[inline]
pub fn is_account<A: AsRef<AccountName>>(account: A) -> bool {
    let a = account.as_ref().as_u64();
    unsafe { eosio_cdt_sys::is_account(a) }
}

/// Verifies that `name` exists in the set of provided auths on a action. Throws
/// if not found.
#[inline]
pub fn require_auth<A: AsRef<AccountName>>(account: A) {
    let a = account.as_ref().as_u64();
    unsafe { eosio_cdt_sys::require_auth(a) }
}

#[inline]
pub fn require_perm<A, P>(account: A, permission: P)
where
    A: AsRef<AccountName>,
    P: AsRef<PermissionName>,
{
    let a = account.as_ref().as_u64();
    let p = permission.as_ref().as_u64();
    unsafe { eosio_cdt_sys::require_auth2(a, p) }
}

#[inline]
pub fn require_level<L: AsRef<PermissionLevel>>(level: L) {
    let level = level.as_ref();
    require_perm(&level.actor, &level.permission)
}

/// Verifies that `name` exists in the set of provided auths on a action. Throws
/// if not found.
#[inline]
pub fn require_permission<A, P>(account: A, permission: P)
where
    A: AsRef<AccountName>,
    P: AsRef<PermissionName>,
{
    let a = account.as_ref().as_u64();
    let p = permission.as_ref().as_u64();
    unsafe { eosio_cdt_sys::require_auth2(a, p) }
}

/// Add the specified account to set of accounts to be notified
#[inline]
pub fn require_recipient<A: AsRef<AccountName>>(account: A) {
    let a = account.as_ref().as_u64();
    unsafe { eosio_cdt_sys::require_recipient(a) }
}
