use super::SecondaryKey;

/// TODO docs
#[derive(Default, Clone, Copy)]
pub struct SecondaryKeys([Option<SecondaryKey>; 16]);

impl From<[Option<SecondaryKey>; 16]> for SecondaryKeys {
    #[must_use]
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

impl From<()> for SecondaryKeys {
    #[must_use]
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
