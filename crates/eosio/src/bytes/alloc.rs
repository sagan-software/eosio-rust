use super::{NumBytes, Read, ReadError, Write, WriteError};
use std::collections::VecDeque;

impl NumBytes for String {
    #[inline]
    fn num_bytes(&self) -> usize {
        self.as_str().num_bytes()
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

impl Write for String {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.as_bytes().write(bytes, pos)
    }
}

impl<T> NumBytes for Vec<T>
where
    T: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        self.as_slice().num_bytes()
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

impl<T> Write for Vec<T>
where
    T: Write,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.as_slice().write(bytes, pos)
    }
}

impl<T> NumBytes for VecDeque<T>
where
    T: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        let mut count = self.len().num_bytes();
        for item in self.iter() {
            count += item.num_bytes();
        }
        count
    }
}

impl<T> Read for VecDeque<T>
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

impl<T> Write for VecDeque<T>
where
    T: Write,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.len().write(bytes, pos)?;
        for item in self.iter() {
            item.write(bytes, pos)?;
        }
        Ok(())
    }
}

// TODO BinaryHeap
// TODO BTreeMap
// TODO BTreeSet
// TODO HashMap
// TODO HashSet
// TODO LinkedList
