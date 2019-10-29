use super::{NumBytes, Read, ReadError, Write, WriteError};

impl<T> NumBytes for Option<T>
where
    T: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        let mut count = self.is_some().num_bytes();
        if let Some(t) = self {
            count += t.num_bytes();
        }
        count
    }
}

impl<T> Read for Option<T>
where
    T: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let is_some = bool::read(bytes, pos)?;
        if is_some {
            Ok(Some(T::read(bytes, pos)?))
        } else {
            Ok(None)
        }
    }
}

impl<T> Write for Option<T>
where
    T: Write + Default,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.is_some().write(bytes, pos)?;
        match self {
            Some(item) => item.write(bytes, pos),
            None => Ok(()),
        }
    }
}
