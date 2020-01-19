use super::{NumBytes, Read, ReadError, Write, WriteError};
use crate::varint::{SignedInt, UnsignedInt};
use core::convert::TryInto;

macro_rules! impl_nums {
    ($($t:ty, $s:expr)*) => ($(
        impl NumBytes for $t
        {
            #[inline]
            #[must_use]
            fn num_bytes(&self) -> usize {
                $s
            }
        }

        #[allow(clippy::use_self)]
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

        #[allow(clippy::use_self)]
        impl Write for $t
        {
            #[inline]
            fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
                let width: usize = $s;
                let ff = <Self as From<u8>>::from(0xff);

                for i in 0..width {
                    // TODO rework this to dynamically allocate?
                    // std::println!("!!! width: {}, pos: {}, bytes len: {}", width, pos, bytes.len());
                    match bytes.get_mut(*pos) {
                        Some(byte) => {
                            let shift = <Self as From<u8>>::from(i as u8).saturating_mul(<Self as From<u8>>::from(8_u8));
                            // TODO when try_into is stablized:
                            let result = ((*self >> shift) & ff).try_into();
                            match result {
                                Ok(b) => *byte = b,
                                Err(_) => return Err(WriteError::TryFromIntError),
                            }
                            // *byte = ((*self >> shift) & ff) as u8;
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

impl_nums! {
    i16, 2
    i32, 4
    i64, 8
    u16, 2
    u32, 4
    u64, 8
    u8, 1
} // TODO i8 u128 i128

impl NumBytes for f32 {
    #[inline]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    fn num_bytes(&self) -> usize {
        UnsignedInt::from(*self).num_bytes()
    }
}

impl Read for usize {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        UnsignedInt::read(bytes, pos).map(Self::from)
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

impl NumBytes for isize {
    #[inline]
    #[must_use]
    fn num_bytes(&self) -> usize {
        SignedInt::from(*self).num_bytes()
    }
}

impl Read for isize {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        SignedInt::read(bytes, pos).map(Self::from)
    }
}

impl Write for isize {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        SignedInt::from(*self).write(bytes, pos)
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

impl<'a> NumBytes for &str {
    #[inline]
    #[must_use]
    fn num_bytes(&self) -> usize {
        let len = self.len();
        len.num_bytes() + len
    }
}

impl<'a> Write for &str {
    #[inline]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        self.as_bytes().write(bytes, pos)
    }
}

macro_rules! for_each_tuple {
    ( $m:ident ) => {
        $m!(A,0);
        $m!(A,0 B,1);
        $m!(A,0 B,1 C,2);
        $m!(A,0 B,1 C,2 D,3);
        $m!(A,0 B,1 C,2 D,3 E,4);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6 H,7);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6 H,7 I,8);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6 H,7 I,8 J,9);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6 H,7 I,8 J,9 K,10);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6 H,7 I,8 J,9 K,10 L,11);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6 H,7 I,8 J,9 K,10 L,11 M,12);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6 H,7 I,8 J,9 K,10 L,11 M,12 N,13);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6 H,7 I,8 J,9 K,10 L,11 M,12 N,13 O,14);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6 H,7 I,8 J,9 K,10 L,11 M,12 N,13 O,14 P,15);
        $m!(A,0 B,1 C,2 D,3 E,4 F,5 G,6 H,7 I,8 J,9 K,10 L,11 M,12 N,13 O,14 P,15 Q,16);
    };
}

macro_rules! tuple_index {
    ($x:expr) => {
        $x
    };
    ($tuple:expr, $index:tt) => {
        tuple_index!($tuple.$index)
    };
}

macro_rules! impl_byte_traits {
    ( $($name:ident,$index:tt)* ) => (
        impl<$($name: NumBytes,)*> NumBytes for ($($name,)*)
        {
            #[inline]
            fn num_bytes(&self) -> usize {
                0_usize
                $(.saturating_add(tuple_index!(self, $index).num_bytes()))*
            }
        }

        impl<$($name: Read,)*> Read for ($($name,)*)
        {
            #[inline]
            fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
                Ok(($($name::read(bytes, pos)?,)*))
            }
        }

        impl<$($name: Write,)*> Write for ($($name,)*)
        {
            #[inline]
            fn write(
                &self,
                bytes: &mut [u8],
                pos: &mut usize,
            ) -> Result<(), WriteError> {
                $(tuple_index!(self, $index).write(bytes, pos)?;)*
                Ok(())
            }
        }
    );
}

for_each_tuple!(impl_byte_traits);
