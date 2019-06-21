//! Traits and implementations related to counting the number of bytes a type
//! requires.

/// Count the number of bytes a type is expected to use.
pub trait NumBytes {
    /// Count the number of bytes a type is expected to use.
    fn num_bytes(&self) -> usize;
}

impl NumBytes for f32 {
    #[inline]
    fn num_bytes(&self) -> usize {
        4
    }
}

impl NumBytes for f64 {
    #[inline]
    fn num_bytes(&self) -> usize {
        8
    }
}

impl NumBytes for bool {
    #[inline]
    fn num_bytes(&self) -> usize {
        1
    }
}

impl NumBytes for char {
    #[inline]
    fn num_bytes(&self) -> usize {
        1
    }
}

impl NumBytes for usize {
    #[inline]
    fn num_bytes(&self) -> usize {
        1
    }
}

impl<T> NumBytes for Vec<T>
where
    T: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        let mut count = 1;
        for item in self.iter() {
            count += item.num_bytes();
        }
        count
    }
}

impl<T> NumBytes for std::collections::VecDeque<T>
where
    T: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        let mut count = 1;
        for item in self.iter() {
            count += item.num_bytes();
        }
        count
    }
}

impl<T> NumBytes for &[T]
where
    T: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        let mut count = 1;
        for item in self.iter() {
            count += item.num_bytes();
        }
        count
    }
}

impl<T> NumBytes for Option<T>
where
    T: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        let mut count = 1;
        if let Some(t) = self {
            count += t.num_bytes();
        }
        count
    }
}

impl NumBytes for String {
    #[inline]
    fn num_bytes(&self) -> usize {
        self.len().saturating_add(1) // TODO: utf16?
    }
}

impl<'a> NumBytes for &'a str {
    #[inline]
    fn num_bytes(&self) -> usize {
        self.len().saturating_add(1) // TODO: utf16?
    }
}

impl<A, B> NumBytes for (A, B)
where
    A: NumBytes,
    B: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        self.0.num_bytes().saturating_add(self.1.num_bytes())
    }
}

impl<A, B, C> NumBytes for (A, B, C)
where
    A: NumBytes,
    B: NumBytes,
    C: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        self.0
            .num_bytes()
            .saturating_add(self.1.num_bytes())
            .saturating_add(self.2.num_bytes())
    }
}

impl<A, B, C, D> NumBytes for (A, B, C, D)
where
    A: NumBytes,
    B: NumBytes,
    C: NumBytes,
    D: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        self.0
            .num_bytes()
            .saturating_add(self.1.num_bytes())
            .saturating_add(self.2.num_bytes())
            .saturating_add(self.3.num_bytes())
    }
}
