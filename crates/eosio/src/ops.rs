//! TODO docs
/// TODO docs
pub trait CheckedAdd<Other = Self>: Sized {
    /// TODO docs
    type Output;
    /// TODO docs
    fn checked_add(self, other: Other) -> Self::Output;
}

/// TODO docs
pub trait CheckedSub<Other = Self>: Sized {
    /// TODO docs
    type Output;
    /// TODO docs
    fn checked_sub(self, other: Other) -> Self::Output;
}

/// TODO docs
pub trait CheckedMul<Other = Self>: Sized {
    /// TODO docs
    type Output;
    /// TODO docs
    fn checked_mul(self, other: Other) -> Self::Output;
}

/// TODO docs
pub trait CheckedDiv<Other = Self>: Sized {
    /// TODO docs
    type Output;
    /// TODO docs
    fn checked_div(self, other: Other) -> Self::Output;
}

/// TODO docs
pub trait CheckedRem<Other = Self>: Sized {
    /// TODO docs
    type Output;
    /// TODO docs
    fn checked_rem(self, other: Other) -> Self::Output;
}
