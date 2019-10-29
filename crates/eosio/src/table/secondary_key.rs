/// TODO docs
#[derive(Clone, Copy, Debug)]
pub enum SecondaryKey {
    /// TODO docs
    U64(u64),
    /// TODO docs
    F64(f64),
    /// TODO docs
    U128(u128),
}

impl From<u64> for SecondaryKey {
    #[must_use]
    fn from(v: u64) -> Self {
        Self::U64(v)
    }
}

impl From<f64> for SecondaryKey {
    #[must_use]
    fn from(v: f64) -> Self {
        Self::F64(v)
    }
}

impl From<u128> for SecondaryKey {
    #[must_use]
    fn from(v: u128) -> Self {
        Self::U128(v)
    }
}

macro_rules! impl_into_type {
    ($($t:ty, $x:ty)*) => ($(
        impl From<$x> for SecondaryKey {
            #[must_use]
            fn from(v: $x) -> Self {
                let v: $t = v.into();
                v.into()
            }
        }
    )*)
}

impl_into_type! {
    u64, u8
    u64, u16
    u64, u32
    // u64, ScopeName
    // u64, TableName
    // u64, crate::account::AccountName
    // u64, crate::action::PermissionName
    // u64, crate::action::ActionName
    // u64, crate::name::Name
}
