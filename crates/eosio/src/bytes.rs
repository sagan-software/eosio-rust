//! TODO docs
use crate::varint::UnsignedInt;
pub use eosio_macros::{NumBytes, Read, Write};
use std::ops::Deref;

/// Count the number of bytes a type is expected to use.
pub trait NumBytes {
    /// Count the number of bytes a type is expected to use.
    fn num_bytes(&self) -> usize;
}

/// Read bytes.
pub trait Read: Sized {
    /// Read bytes.
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError>;
}

/// Error that can be returned when reading bytes.
#[derive(Debug, Clone, Copy)]
pub enum ReadError {
    /// Not enough bytes.
    NotEnoughBytes,
}

/// Write bytes.
pub trait Write: Sized {
    /// Write bytes.
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError>;
}

/// Error that can be returned when writing bytes.
#[derive(Debug, Clone, Copy)]
pub enum WriteError {
    /// Not enough space in the vector.
    NotEnoughSpace,
    /// Failed to parse an integer.
    TryFromIntError,
}

macro_rules! impl_num {
    ($($t:ty, $s:expr)*) => ($(
        impl NumBytes for $t
        {
            #[inline]
            fn num_bytes(&self) -> usize {
                $s
            }
        }

        impl Read for $t {
            #[inline]
            fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
                let width: usize = $s;

                let mut num = <Self as From<u8>>::from(0_u8);
                for i in 0..width {
                    match bytes.get(*pos) {
                        Some(b) => {
                            let shift = <Self as From<u8>>::from(i as u8).saturating_mul(<Self as From<u8>>::from(8_u8));
                            num |= <Self as From<u8>>::from(*b) << shift;
                        }
                        None => return Err(ReadError::NotEnoughBytes),
                    }
                    *pos = pos.saturating_add(1);
                }
                Ok(num)
            }
        }

        impl Write for $t
        {
            #[inline]
            fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
                let width: usize = $s;
                let ff = <Self as From<u8>>::from(0xff);

                for i in 0..width {
                    // TODO rework this to dynamically allocate?
                    match bytes.get_mut(*pos) {
                        Some(byte) => {
                            let shift = <Self as From<u8>>::from(i as u8).saturating_mul(<Self as From<u8>>::from(8_u8));
                            // TODO when try_into is stablized:
                            // let result = ((*self >> shift) & ff).try_into();
                            // match result {
                            //     Ok(b) => *byte = b,
                            //     Err(_) => return Err(WriteError::TryFromIntError),
                            // }
                            *byte = ((*self >> shift) & ff) as u8;
                        }
                        None => return Err(WriteError::NotEnoughSpace),
                    }
                    *pos = pos.saturating_add(1);
                }

                Ok(())
            }
        }
    )*)
}

impl_num!(
    u8, 1
    u16, 2
    i16, 2
    u32, 4
    i32, 4
    u64, 8
    i64, 8
); // TODO i8 u128 i128

impl NumBytes for f32 {
    #[inline]
    fn num_bytes(&self) -> usize {
        4
    }
}

impl Read for f32 {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let bits = u32::read(bytes, pos)?;
        let num = Self::from_bits(bits);
        Ok(num)
    }
}

impl Write for f32 {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.to_bits().write(bytes, pos)
    }
}

impl NumBytes for f64 {
    #[inline]
    fn num_bytes(&self) -> usize {
        8
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

impl Write for f64 {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.to_bits().write(bytes, pos)
    }
}

impl NumBytes for bool {
    #[inline]
    fn num_bytes(&self) -> usize {
        1
    }
}

impl Read for bool {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u8::read(bytes, pos).map(|v| v == 1)
    }
}

impl Write for bool {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        let value: u8 = if *self { 1 } else { 0 };
        value.write(bytes, pos)
    }
}

impl NumBytes for char {
    #[inline]
    fn num_bytes(&self) -> usize {
        1
    }
}

impl Read for char {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u8::read(bytes, pos).map(|v| v as Self)
    }
}

impl Write for char {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        (*self as u8).write(bytes, pos)
    }
}

impl NumBytes for usize {
    #[inline]
    fn num_bytes(&self) -> usize {
        UnsignedInt::from(*self).num_bytes()
    }
}

impl Read for usize {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        UnsignedInt::read(bytes, pos).map(std::convert::Into::into)
    }
}

impl Write for usize {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        UnsignedInt::from(*self).write(bytes, pos)
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

impl<T> NumBytes for std::collections::VecDeque<T>
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

impl<T> Read for std::collections::VecDeque<T>
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

impl<T> Write for std::collections::VecDeque<T>
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

impl<T> NumBytes for &[T]
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

impl<T> Write for &[T]
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

impl<'a> NumBytes for &'a str {
    #[inline]
    fn num_bytes(&self) -> usize {
        let len = self.len();
        len.num_bytes() + len
    }
}

impl<'a> Write for &'a str {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.as_bytes().write(bytes, pos)
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

impl<A, B> Write for (A, B)
where
    A: Write,
    B: Write,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.0.write(bytes, pos)?;
        self.1.write(bytes, pos)?;
        Ok(())
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

impl<A, B, C> Write for (A, B, C)
where
    A: Write,
    B: Write,
    C: Write,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.0.write(bytes, pos)?;
        self.1.write(bytes, pos)?;
        self.2.write(bytes, pos)?;
        Ok(())
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

impl<A, B, C, D> Write for (A, B, C, D)
where
    A: Write,
    B: Write,
    C: Write,
    D: Write,
{
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.0.write(bytes, pos)?;
        self.1.write(bytes, pos)?;
        self.2.write(bytes, pos)?;
        self.3.write(bytes, pos)?;
        Ok(())
    }
}

macro_rules! impl_array {
    ($($x:expr)*) => ($(
        impl<T> NumBytes for [T; $x]
        where
            T: NumBytes,
        {
            #[inline]
            fn num_bytes(&self) -> usize {
                // let mut count = 1;
                let mut count = 0;
                for item in self.iter() {
                    count += item.num_bytes();
                }
                count
            }
        }

        impl<T> Read for [T; $x]
        where
            T: Read + Default + Copy,
        {
            #[inline]
            fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
                // usize::read(bytes, pos)?;

                let mut items = [T::default(); $x];
                for item in items.iter_mut() {
                    let v = T::read(bytes, pos)?;
                    *item = v;
                }

                Ok(items)
            }
        }

        impl<T> Write for [T; $x]
        where
            T: Write,
        {
            #[inline]
            fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
                // (&self[..]).write(bytes, pos)
                for item in self.iter() {
                    item.write(bytes, pos)?;
                }
                Ok(())
            }
        }
    )*)
}

impl_array! {
     1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40
    41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60
    61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80
    81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100
}

#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! test_type {
        ($($i:ident, $t:ty, $e:expr)*) => ($(
            #[test]
            #[allow(clippy::float_cmp)]
            fn $i() {
                let expected_pos = $e.num_bytes();
                let mut bytes = [0_u8; 100];
                let thing: $t = $e;

                let mut write_pos = 0;
                thing.write(&mut bytes, &mut write_pos).unwrap();
                assert_eq!(expected_pos, write_pos);

                let mut read_pos = 0;
                let result = <$t as Read>::read(&bytes, &mut read_pos).unwrap();
                assert_eq!(expected_pos, read_pos);

                assert_eq!($e, result);
                assert_eq!(write_pos, read_pos);
            }
        )*)
    }

    test_type!(
        test_u8, u8, 1_u8
        test_u16, u16, 1_u16
        test_u32, u32, 1_u32
        test_u64, u64, 1_u64
        // test_i8, i8, 1i8
        test_i16, i16, -1_i16
        test_i32, i32, -1_i32
        test_i64, i64, -1_i64
        test_bool_true, bool, true
        test_bool_false, bool, false
        // test_option_none, Option<u8>, None as Option<u8>
        // test_option_some, Option<u8>, Some(1)
        test_string, String, "neat".to_string()
        test_vec, Vec<u8>, vec![1_u8, 2_u8, 3_u8]
        test_tuple2, (u8, u16), (1_u8, 2_u16)
        test_tuple3, (u8, u16, u32), (1_u8, 2_u16, 3_u32)
        test_tuple4, (u8, u16, u32, u64), (1_u8, 2_u16, 3_u32, 4_u64)
        test_array1, [u8; 1], [1_u8; 1]
        test_array2, [u8; 2], [1_u8; 2]
        test_array3, [u8; 3], [1_u8; 3]
        test_array4, [u8; 4], [1_u8; 4]
        test_array5, [u8; 5], [1_u8; 5]
        test_array6, [u8; 6], [1_u8; 6]
        test_array7, [u8; 7], [1_u8; 7]
        test_array8, [u8; 8], [1_u8; 8]
        test_array9, [u8; 9], [1_u8; 9]
        test_array10, [u8; 10], [1_u8; 10]
        test_array11, [u8; 11], [1_u8; 11]
        test_array12, [u8; 12], [1_u8; 12]
        test_array13, [u8; 13], [1_u8; 13]
        test_array14, [u8; 14], [1_u8; 14]
        test_array15, [u8; 15], [1_u8; 15]
        test_array16, [u8; 16], [1_u8; 16]
        test_array17, [u8; 17], [1_u8; 17]
        test_array18, [u8; 18], [1_u8; 18]
        test_array19, [u8; 19], [1_u8; 19]
        test_array20, [u8; 20], [1_u8; 20]
        test_f32, f32, -0.12345_f32
        test_f64, f64, -0.12345_f64
    );

    // #[test]
    // fn test_struct_named_fields() {
    //     #[derive(Read, Write, PartialEq, Debug)]
    //     struct Thing {
    //         a: u64,
    //         b: u64,
    //         c: u32,
    //     }

    //     let thing1 = Thing { a: 1, b: 2, c: 3 };

    //     let mut bytes = [0u8; 100];
    //     let mut write_pos = 0;
    //     thing1.write(&mut bytes, &mut write_pos).unwrap();
    //     assert_eq!(write_pos, 20);

    //     let mut read_pos = 0;
    //     let thing2 = Thing::read(&bytes, &mut read_pos).unwrap();
    //     assert_eq!(read_pos, write_pos);

    //     assert_eq!(thing1, thing2);
    //     assert_eq!(thing1.a, 1);
    //     assert_eq!(thing1.b, 2);
    //     assert_eq!(thing1.c, 3);
    // }

    // #[test]
    // fn test_struct_unnamed_fields() {
    //     #[derive(Read, Write, PartialEq, Debug)]
    //     struct Thing(u64, u64, u32);

    //     let thing1 = Thing(1, 2, 3);

    //     let mut bytes = [0u8; 100];

    //     let mut write_pos = 0;
    //     thing1.write(&mut bytes, &mut write_pos).unwrap();
    //     assert_eq!(write_pos, 20);

    //     let mut read_pos = 0;
    //     let thing2 = Thing::read(&bytes, &mut read_pos).unwrap();
    //     assert_eq!(read_pos, write_pos);

    //     assert_eq!(thing1, thing2);
    //     assert_eq!(thing1.0, 1);
    //     assert_eq!(thing1.1, 2);
    //     assert_eq!(thing1.2, 3);
    // }

    #[test]
    #[allow(clippy::result_unwrap_used)]
    fn test_read_pos() {
        let bytes = &[
            10, 9, 0, 1, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 20, 4, 3, 2, 1, 1,
            1, 1, 1,
        ];

        let mut pos = 0;
        u8::read(bytes, &mut pos).unwrap();
        assert_eq!(pos, 1);

        u8::read(bytes, &mut pos).unwrap();
        assert_eq!(pos, 2);

        u16::read(bytes, &mut pos).unwrap();
        assert_eq!(pos, 4);

        u32::read(bytes, &mut pos).unwrap();
        assert_eq!(pos, 8);

        u64::read(bytes, &mut pos).unwrap();
        assert_eq!(pos, 16);

        u64::read(bytes, &mut pos).unwrap();
        assert_eq!(pos, 24);

        pos = 2;
        u64::read(bytes, &mut pos).unwrap();
        assert_eq!(pos, 10);

        pos = 10;
        u64::read(bytes, &mut pos).unwrap();
        assert_eq!(pos, 18);
    }

    #[test]
    #[allow(clippy::result_unwrap_used)]
    fn test_write_pos() {
        let bytes = &mut [0u8; 1000];

        let mut pos = 0;
        1u8.write(bytes, &mut pos).unwrap();
        assert_eq!(pos, 1);

        1_u16.write(bytes, &mut pos).unwrap();
        assert_eq!(pos, 3);

        1_u32.write(bytes, &mut pos).unwrap();
        assert_eq!(pos, 7);

        1_u64.write(bytes, &mut pos).unwrap();
        assert_eq!(pos, 15);
    }
}

/// A stream of bytes
pub struct DataStream {
    /// TODO docs
    bytes: Vec<u8>,
    /// TODO docs
    pos: usize,
}

impl DataStream {
    /// Read something from the stream
    #[inline]
    pub fn read<T: Read>(&mut self) -> Result<T, ReadError> {
        let bytes = self
            .bytes
            .get(self.pos..)
            .ok_or(ReadError::NotEnoughBytes)?;
        T::read(bytes, &mut self.pos)
    }

    /// Write something to the stream
    #[inline]
    pub fn write<T: Write>(&mut self, thing: &T) -> Result<(), WriteError> {
        let bytes = self
            .bytes
            .get_mut(self.pos..)
            .ok_or(WriteError::NotEnoughSpace)?;
        thing.write(bytes, &mut self.pos)
    }

    /// Gets bytes as slice
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.get(self.pos..).unwrap_or_else(|| &self.bytes)
    }

    /// Resets the data stream position
    #[inline]
    pub fn reset(&mut self) {
        self.pos = 0;
    }

    /// Get the current position
    #[inline]
    pub const fn position(&self) -> usize {
        self.pos
    }

    /// Gets the remaining number of bytes
    #[inline]
    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.pos
    }
}

impl From<Vec<u8>> for DataStream {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes, pos: 0 }
    }
}

impl Deref for DataStream {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}
