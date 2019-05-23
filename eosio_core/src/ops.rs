pub trait CheckedAdd<Other = Self>: Sized {
    type Output;
    fn checked_add(self, other: Other) -> Self::Output;
}

pub trait CheckedSub<Other = Self>: Sized {
    type Output;
    fn checked_sub(self, other: Other) -> Self::Output;
}

pub trait CheckedMul<Other = Self>: Sized {
    type Output;
    fn checked_mul(self, other: Other) -> Self::Output;
}

pub trait CheckedDiv<Other = Self>: Sized {
    type Output;
    fn checked_div(self, other: Other) -> Self::Output;
}

pub trait CheckedRem<Other = Self>: Sized {
    type Output;
    fn checked_rem(self, other: Other) -> Self::Output;
}
