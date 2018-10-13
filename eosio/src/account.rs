use eosio_macros::*;
use time::Time;

eosio_name!(AccountName);
eosio_name!(PermissionName);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Read, Write, Hash, PartialOrd, Ord)]
pub struct PermissionLevel {
    pub account: AccountName,
    pub permission: PermissionName,
}

impl AccountName {
    /// Get the current receiver of the action.
    pub fn receiver() -> Self {
        let name = unsafe { ::eosio_sys::current_receiver() };
        name.into()
    }

    /// Verifies that `name` name has auth.
    pub fn has_auth(&self) -> bool {
        unsafe { ::eosio_sys::has_auth(self.0) }
    }

    /// Check if an account is privileged
    pub fn is_privileged(&self) -> bool {
        unsafe { ::eosio_sys::is_privileged(self.0) }
    }

    /// Verifies that `name` is an account.
    pub fn is_account(&self) -> bool {
        unsafe { ::eosio_sys::is_account(self.0) }
    }

    /// Returns the creation time of an account
    pub fn creation_time(&self) -> Time {
        let time = unsafe { ::eosio_sys::get_account_creation_time(self.0) };
        time.into()
    }

    /// Returns the last used time of a permission
    pub fn permission_last_used<P>(&self, permission: P) -> Time
    where
        P: Into<PermissionName>,
    {
        let p = permission.into();
        let time = unsafe { ::eosio_sys::get_permission_last_used(self.0, p.into()) };
        time.into()
    }

    /// Verifies that `name` exists in the set of provided auths on a action. Throws if not found.
    pub fn require_auth(&self) {
        unsafe { ::eosio_sys::require_auth(self.0) }
    }

    /// Verifies that `name` exists in the set of provided auths on a action. Throws if not found.
    pub fn require_permission<P>(&self, permission: P)
    where
        P: Into<PermissionName>,
    {
        let p = permission.into();
        unsafe { ::eosio_sys::require_auth2(self.0, p.into()) }
    }

    /// Add the specified account to set of accounts to be notified
    pub fn require_recipient(&self) {
        unsafe { ::eosio_sys::require_recipient(self.0) }
    }
}

/// Verifies that `name` exists in the set of provided auths on a action. Throws if not found.
pub fn require_auth<A>(account: A)
where
    A: Into<AccountName>,
{
    account.into().require_auth()
}

/// Verifies that `name` exists in the set of provided auths on a action. Throws if not found.
pub fn require_permission<A, P>(account: A, permission: P)
where
    A: Into<AccountName>,
    P: Into<PermissionName>,
{
    account.into().require_permission(permission)
}

/// Add the specified account to set of accounts to be notified
pub fn require_recipient<A>(account: A)
where
    A: Into<AccountName>,
{
    account.into().require_recipient()
}
