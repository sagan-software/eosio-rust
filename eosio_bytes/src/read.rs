//! Traits, types, and implementations related to reading bytes.

/// Error that can be returned when reading bytes.
#[derive(Debug, Clone, Copy)]
pub enum ReadError {
    /// Not enough bytes.
    NotEnoughBytes,
}

/// Read bytes.
pub trait Read: Sized {
    /// Read bytes.
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError>;
}

impl Read for f32 {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let bits = u32::read(bytes, pos)?;
        let num = Self::from_bits(bits);
        Ok(num)
    }
}

impl Read for f64 {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let bits = u64::read(bytes, pos)?;
        let num = Self::from_bits(bits);
        Ok(num)
    }
}

impl Read for bool {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u8::read(bytes, pos).map(|v| v == 1)
    }
}

impl Read for char {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u8::read(bytes, pos).map(|v| v as Self)
    }
}

impl Read for usize {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        // TODO: fix this. usize isn't always u8?
        u8::read(bytes, pos).map(|v| v as Self)
    }
}

impl<T> Read for Option<T>
where
    T: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let is_some = bool::read(bytes, pos)?;
        let item = T::read(bytes, pos)?;
        let opt = if is_some { Some(item) } else { None };
        Ok(opt)
    }
}

impl<T> Read for Vec<T>
where
    T: Read + Default + Clone,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let capacity = usize::read(bytes, pos)?;

        let mut results = Self::new();
        results.resize(capacity, T::default());

        for item in &mut results {
            let r = T::read(bytes, pos)?;
            *item = r;
        }

        Ok(results)
    }
}

impl Read for String {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        // TODO: may need to read this as a cstr
        let utf8 = Vec::<u8>::read(bytes, pos)?;
        let s = Self::from_utf8_lossy(&utf8);
        Ok(s.into_owned())
    }
}

impl<A, B> Read for (A, B)
where
    A: Read,
    B: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let a = A::read(bytes, pos)?;
        let b = B::read(bytes, pos)?;
        Ok((a, b))
    }
}

impl<A, B, C> Read for (A, B, C)
where
    A: Read,
    B: Read,
    C: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let a = A::read(bytes, pos)?;
        let b = B::read(bytes, pos)?;
        let c = C::read(bytes, pos)?;
        Ok((a, b, c))
    }
}

impl<A, B, C, D> Read for (A, B, C, D)
where
    A: Read,
    B: Read,
    C: Read,
    D: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let a = A::read(bytes, pos)?;
        let b = B::read(bytes, pos)?;
        let c = C::read(bytes, pos)?;
        let d = D::read(bytes, pos)?;
        Ok((a, b, c, d))
    }
}
