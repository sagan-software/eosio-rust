use crate::{NumBytes, Read, ReadError, Write, WriteError};
use alloc::collections::VecDeque;

impl<T> NumBytes for VecDeque<T>
where
    T: NumBytes,
{
    #[inline]
    #[must_use]
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
