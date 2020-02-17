use crate::{NumBytes, Read, ReadError, Write, WriteError};

#[derive(Clone, Default, Debug)]
pub struct BinaryExtension<T>(Option<T>);

impl<T> NumBytes for BinaryExtension<T>
where
    T: NumBytes,
{
    fn num_bytes(&self) -> usize {
        self.0.as_ref().map(|t| t.num_bytes()).unwrap_or_default()
    }
}

impl<T> Read for BinaryExtension<T>
where
    T: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        if bytes.len() - *pos > 0 {
            T::read(bytes, pos).map(|t| Self(Some(t)))
        } else {
            Ok(Self(None))
        }
    }
}

impl<T> Write for BinaryExtension<T>
where
    T: Write,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        match &self.0 {
            Some(t) => t.write(bytes, pos),
            None => Ok(()),
        }
    }
}

impl<T> BinaryExtension<T> {
    #[inline]
    pub const fn new(t: Option<T>) -> Self {
        Self(t)
    }

    #[inline]
    pub fn as_value(&self) -> Option<&T> {
        self.0.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    pub fn into_value(self) -> Option<T> {
        self.0
    }
}
