#[cfg(feature = "contract")]
use crate::time::Time;
use crate::{n, name};
use eosio_bytes::{NumBytes, Read, Write};

name!(AccountName);
name!(PermissionName);

/// A permission
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Read,
    Write,
    NumBytes,
    Hash,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Authorization {
    /// Docs
    pub actor: AccountName,
    /// Docs
    pub permission: PermissionName,
}

// #[cfg(feature = "stdweb")]
// stdweb::js_serializable!(Authorization);
// #[cfg(feature = "stdweb")]
// stdweb::js_deserializable!(Authorization);

impl Authorization {
    /// Returns the 'active' authorization for an account.
    #[inline]
    pub fn active(actor: AccountName) -> Self {
        Self {
            actor,
            permission: n!(active).into(),
        }
    }

    /// Returns the 'owner' authorization for an account
    #[inline]
    pub fn owner(actor: AccountName) -> Self {
        Self {
            actor,
            permission: n!(owner).into(),
        }
    }
}

/// RAM in bytes
pub struct RamBytes(i64);

/// Net Weight
pub struct NetWeight(i64);

/// CPU Weight
pub struct CpuWeight(i64);

impl AccountName {
    /// Get the current receiver of the action.
    #[cfg(feature = "contract")]
    #[inline]
    pub fn receiver() -> Self {
        let name = unsafe { ::eosio_cdt_sys::current_receiver() };
        name.into()
    }
}

/// Returns the creation time of an account
#[cfg(feature = "contract")]
#[inline]
pub fn creation_time<A: Into<AccountName>>(account: A) -> Time {
    let a = account.into();
    let time = unsafe { ::eosio_cdt_sys::get_account_creation_time(a.into()) };
    time.into()
}

/// Returns the last used time of a permission
#[cfg(feature = "contract")]
#[inline]
pub fn permission_last_used<A, P>(account: A, permission: P) -> Time
where
    A: Into<AccountName>,
    P: Into<PermissionName>,
{
    let a = account.into();
    let p = permission.into();
    let time = unsafe {
        ::eosio_cdt_sys::get_permission_last_used(a.into(), p.into())
    };
    time.into()
}

// TODO: support chains that have more/less than 21 producers
/// Gets the top 21 producers
#[cfg(feature = "contract")]
#[inline]
pub fn active_producers() -> [Option<AccountName>; 21] {
    let mut producers = [0_u64; 21];
    let producers_ptr: *mut u64 = &mut producers as *mut _ as *mut u64;
    let producers_len: u32 = 168; // 8 * 21;
    unsafe {
        ::eosio_cdt_sys::get_active_producers(producers_ptr, producers_len)
    };

    let mut options = [None; 21];
    for (index, item) in options.iter_mut().enumerate() {
        *item = match producers.get(index) {
            Some(&producer) => {
                if producer == 0 {
                    None
                } else {
                    Some(AccountName(producer))
                }
            }
            None => None,
        }
    }
    options
}

/// Priviledged
#[cfg(feature = "contract")]
#[inline]
pub fn get_resource_limits<A: Into<AccountName>>(
    account: A,
) -> (RamBytes, NetWeight, CpuWeight) {
    let mut ram_bytes = 0_i64;
    let ram_bytes_ptr = &mut ram_bytes as *mut _ as *mut i64;
    let mut net_weight = 0_i64;
    let net_weight_ptr = &mut net_weight as *mut _ as *mut i64;
    let mut cpu_weight = 0_i64;
    let cpu_weight_ptr = &mut cpu_weight as *mut _ as *mut i64;
    let a = account.into();
    unsafe {
        ::eosio_cdt_sys::get_resource_limits(
            a.into(),
            ram_bytes_ptr,
            net_weight_ptr,
            cpu_weight_ptr,
        )
    };
    (
        RamBytes(ram_bytes),
        NetWeight(net_weight),
        CpuWeight(cpu_weight),
    )
}

/// Verifies that `name` name has auth.
#[cfg(feature = "contract")]
#[inline]
pub fn has_auth<A: Into<AccountName>>(account: A) -> bool {
    let a = account.into();
    unsafe { ::eosio_cdt_sys::has_auth(a.into()) }
}

/// Check if an account is privileged
#[cfg(feature = "contract")]
#[inline]
pub fn is_privileged<A: Into<AccountName>>(account: A) -> bool {
    let a = account.into();
    unsafe { ::eosio_cdt_sys::is_privileged(a.into()) }
}

/// Verifies that `name` is an account.
#[cfg(feature = "contract")]
#[inline]
pub fn is_account<A: Into<AccountName>>(account: A) -> bool {
    let a = account.into();
    unsafe { ::eosio_cdt_sys::is_account(a.into()) }
}

/// Verifies that `name` exists in the set of provided auths on a action. Throws if not found.
#[cfg(feature = "contract")]
#[inline]
pub fn require_auth<A: Into<AccountName>>(account: A) {
    let a = account.into();
    unsafe { ::eosio_cdt_sys::require_auth(a.into()) }
}

/// Verifies that `name` exists in the set of provided auths on a action. Throws if not found.
#[cfg(feature = "contract")]
#[inline]
pub fn require_permission<A, P>(account: A, permission: P)
where
    A: Into<AccountName>,
    P: Into<PermissionName>,
{
    let a = account.into();
    let p = permission.into();
    unsafe { ::eosio_cdt_sys::require_auth2(a.into(), p.into()) }
}

/// Add the specified account to set of accounts to be notified
#[cfg(feature = "contract")]
#[inline]
pub fn require_recipient<A: Into<AccountName>>(account: A) {
    let a = account.into();
    unsafe { ::eosio_cdt_sys::require_recipient(a.into()) }
}
