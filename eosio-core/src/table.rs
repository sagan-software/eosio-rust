use crate::{AccountName, NumBytes, Read, ScopeName, TableName, Write};
use std::marker::PhantomData;

/// TODO docs
pub trait Table: Sized {
    /// TODO docs
    const NAME: u64;
    /// TODO docs
    type Row: Read + Write + NumBytes;
    /// TODO docs
    fn primary_key(row: &Self::Row) -> u64;
    /// TODO docs
    fn secondary_keys(_row: &Self::Row) -> SecondaryKeys {
        SecondaryKeys::default()
    }
    /// TODO docs
    #[inline]
    fn table<C, S>(code: C, scope: S) -> PrimaryTableIndex<Self>
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
    {
        PrimaryTableIndex::new(code, scope)
    }
}

/// TODO docs
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash, PartialOrd, Ord)]
pub struct SecondaryTableName(TableName, usize);

impl SecondaryTableName {
    /// TODO docs
    #[inline]
    pub const fn new(primary: TableName, index: usize) -> Self {
        Self(primary, index)
    }

    /// TODO docs
    #[inline]
    pub const fn primary(&self) -> TableName {
        self.0
    }

    /// TODO docs
    #[inline]
    pub const fn index(&self) -> usize {
        self.1
    }
}

impl From<SecondaryTableName> for u64 {
    #[inline]
    fn from(t: SecondaryTableName) -> Self {
        let index = t.1 as Self;
        let table: Self = t.0.into();
        (table & 0xFFFF_FFFF_FFFF_FFF0_u64)
            | (index & 0x0000_0000_0000_000F_u64)
    }
}

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
    fn from(v: u64) -> Self {
        Self::U64(v)
    }
}

impl From<f64> for SecondaryKey {
    fn from(v: f64) -> Self {
        Self::F64(v)
    }
}

impl From<u128> for SecondaryKey {
    fn from(v: u128) -> Self {
        Self::U128(v)
    }
}

macro_rules! impl_into_type {
    ($($t:ty, $x:ty)*) => ($(
        impl From<$x> for SecondaryKey {
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
    u64, crate::Name
    u64, crate::AccountName
    u64, crate::TableName
    u64, crate::PermissionName
    u64, crate::ScopeName
    u64, crate::ActionName
}

/// TODO docs
#[derive(Default, Clone, Copy)]
pub struct SecondaryKeys([Option<SecondaryKey>; 16]);

impl From<[Option<SecondaryKey>; 16]> for SecondaryKeys {
    fn from(v: [Option<SecondaryKey>; 16]) -> Self {
        Self(v)
    }
}

impl SecondaryKeys {
    /// TODO docs
    pub fn iter(&self) -> impl Iterator<Item = &Option<SecondaryKey>> {
        self.0.iter()
    }

    /// TODO docs
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Option<SecondaryKey>> {
        self.0.iter_mut()
    }
}

/// TODO docs
#[derive(Copy, Clone, Debug)]
pub struct PrimaryTableIndex<T>
where
    T: Table,
{
    /// TODO docs
    pub code: AccountName,
    /// TODO docs
    pub scope: ScopeName,
    /// TODO docs
    _data: PhantomData<T>,
}

impl<T> PrimaryTableIndex<T>
where
    T: Table,
{
    /// TODO docs
    #[inline]
    pub fn new<C, S>(code: C, scope: S) -> Self
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
    {
        Self {
            code: code.into(),
            scope: scope.into(),
            _data: PhantomData,
        }
    }
}

/// TODO docs
#[derive(Copy, Clone, Debug)]
pub struct SecondaryTableIndex<K, T>
where
    T: Table,
{
    /// TODO docs
    pub code: AccountName,
    /// TODO docs
    pub scope: ScopeName,
    /// TODO docs
    pub table: SecondaryTableName,
    /// TODO docs
    _data: PhantomData<(K, T)>,
}

impl<K, T> SecondaryTableIndex<K, T>
where
    T: Table,
{
    /// TODO docs
    #[inline]
    pub fn new<C, S, N>(code: C, scope: S, name: N, index: usize) -> Self
    where
        C: Into<AccountName>,
        S: Into<ScopeName>,
        N: Into<TableName>,
    {
        Self {
            code: code.into(),
            scope: scope.into(),
            table: SecondaryTableName::new(name.into(), index),
            _data: PhantomData,
        }
    }

    /// TODO docs
    pub fn primary_index(&self) -> PrimaryTableIndex<T> {
        PrimaryTableIndex::new(self.code, self.scope)
    }
}

impl From<()> for SecondaryKeys {
    fn from(_v: ()) -> Self {
        Self::default()
    }
}

impl<A> From<(A,)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
{
    fn from((a,): (A,)) -> Self {
        Self([
            Some(a.into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

impl<A, B> From<(A, B)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
{
    fn from((a, b): (A, B)) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

impl<A, B, C> From<(A, B, C)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
{
    fn from((a, b, c): (A, B, C)) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

impl<A, B, C, D> From<(A, B, C, D)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names)]
impl<A, B, C, D, E> From<(A, B, C, D, E)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
{
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names)]
impl<A, B, C, D, E, F> From<(A, B, C, D, E, F)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
{
    fn from((a, b, c, d, e, f): (A, B, C, D, E, F)) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names)]
impl<A, B, C, D, E, F, G> From<(A, B, C, D, E, F, G)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
    G: Into<SecondaryKey>,
{
    fn from((a, b, c, d, e, f, g): (A, B, C, D, E, F, G)) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            Some(g.into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names)]
impl<A, B, C, D, E, F, G, H> From<(A, B, C, D, E, F, G, H)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
    G: Into<SecondaryKey>,
    H: Into<SecondaryKey>,
{
    fn from((a, b, c, d, e, f, g, h): (A, B, C, D, E, F, G, H)) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            Some(g.into()),
            Some(h.into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names)]
impl<A, B, C, D, E, F, G, H, I> From<(A, B, C, D, E, F, G, H, I)>
    for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
    G: Into<SecondaryKey>,
    H: Into<SecondaryKey>,
    I: Into<SecondaryKey>,
{
    fn from((a, b, c, d, e, f, g, h, i): (A, B, C, D, E, F, G, H, I)) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            Some(g.into()),
            Some(h.into()),
            Some(i.into()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names)]
impl<A, B, C, D, E, F, G, H, I, J> From<(A, B, C, D, E, F, G, H, I, J)>
    for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
    G: Into<SecondaryKey>,
    H: Into<SecondaryKey>,
    I: Into<SecondaryKey>,
    J: Into<SecondaryKey>,
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j): (A, B, C, D, E, F, G, H, I, J),
    ) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            Some(g.into()),
            Some(h.into()),
            Some(i.into()),
            Some(j.into()),
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names)]
impl<A, B, C, D, E, F, G, H, I, J, K> From<(A, B, C, D, E, F, G, H, I, J, K)>
    for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
    G: Into<SecondaryKey>,
    H: Into<SecondaryKey>,
    I: Into<SecondaryKey>,
    J: Into<SecondaryKey>,
    K: Into<SecondaryKey>,
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k): (A, B, C, D, E, F, G, H, I, J, K),
    ) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            Some(g.into()),
            Some(h.into()),
            Some(i.into()),
            Some(j.into()),
            Some(k.into()),
            None,
            None,
            None,
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names)]
impl<A, B, C, D, E, F, G, H, I, J, K, L>
    From<(A, B, C, D, E, F, G, H, I, J, K, L)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
    G: Into<SecondaryKey>,
    H: Into<SecondaryKey>,
    I: Into<SecondaryKey>,
    J: Into<SecondaryKey>,
    K: Into<SecondaryKey>,
    L: Into<SecondaryKey>,
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
        ),
    ) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            Some(g.into()),
            Some(h.into()),
            Some(i.into()),
            Some(j.into()),
            Some(k.into()),
            Some(l.into()),
            None,
            None,
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names, clippy::type_complexity)]
impl<A, B, C, D, E, F, G, H, I, J, K, L, M>
    From<(A, B, C, D, E, F, G, H, I, J, K, L, M)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
    G: Into<SecondaryKey>,
    H: Into<SecondaryKey>,
    I: Into<SecondaryKey>,
    J: Into<SecondaryKey>,
    K: Into<SecondaryKey>,
    L: Into<SecondaryKey>,
    M: Into<SecondaryKey>,
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
        ),
    ) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            Some(g.into()),
            Some(h.into()),
            Some(i.into()),
            Some(j.into()),
            Some(k.into()),
            Some(l.into()),
            Some(m.into()),
            None,
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names, clippy::type_complexity)]
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N>
    From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
    G: Into<SecondaryKey>,
    H: Into<SecondaryKey>,
    I: Into<SecondaryKey>,
    J: Into<SecondaryKey>,
    K: Into<SecondaryKey>,
    L: Into<SecondaryKey>,
    M: Into<SecondaryKey>,
    N: Into<SecondaryKey>,
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
        ),
    ) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            Some(g.into()),
            Some(h.into()),
            Some(i.into()),
            Some(j.into()),
            Some(k.into()),
            Some(l.into()),
            Some(m.into()),
            Some(n.into()),
            None,
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names, clippy::type_complexity)]
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O>
    From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
    G: Into<SecondaryKey>,
    H: Into<SecondaryKey>,
    I: Into<SecondaryKey>,
    J: Into<SecondaryKey>,
    K: Into<SecondaryKey>,
    L: Into<SecondaryKey>,
    M: Into<SecondaryKey>,
    N: Into<SecondaryKey>,
    O: Into<SecondaryKey>,
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
        ),
    ) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            Some(g.into()),
            Some(h.into()),
            Some(i.into()),
            Some(j.into()),
            Some(k.into()),
            Some(l.into()),
            Some(m.into()),
            Some(n.into()),
            Some(o.into()),
            None,
        ])
    }
}

#[allow(clippy::many_single_char_names, clippy::type_complexity)]
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P>
    From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)> for SecondaryKeys
where
    A: Into<SecondaryKey>,
    B: Into<SecondaryKey>,
    C: Into<SecondaryKey>,
    D: Into<SecondaryKey>,
    E: Into<SecondaryKey>,
    F: Into<SecondaryKey>,
    G: Into<SecondaryKey>,
    H: Into<SecondaryKey>,
    I: Into<SecondaryKey>,
    J: Into<SecondaryKey>,
    K: Into<SecondaryKey>,
    L: Into<SecondaryKey>,
    M: Into<SecondaryKey>,
    N: Into<SecondaryKey>,
    O: Into<SecondaryKey>,
    P: Into<SecondaryKey>,
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
        ),
    ) -> Self {
        Self([
            Some(a.into()),
            Some(b.into()),
            Some(c.into()),
            Some(d.into()),
            Some(e.into()),
            Some(f.into()),
            Some(g.into()),
            Some(h.into()),
            Some(i.into()),
            Some(j.into()),
            Some(k.into()),
            Some(l.into()),
            Some(m.into()),
            Some(n.into()),
            Some(o.into()),
            Some(p.into()),
        ])
    }
}
