//! Implementations for various types.

use crate::*;

macro_rules! impl_num {
    ($($t:ty, $s:expr)*) => ($(
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

        impl NumBytes for $t
        {
            #[inline]
            fn num_bytes(&self) -> usize {
                $s
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

macro_rules! impl_array {
    ($($x:expr)*) => ($(
        impl<T> Read for [T; $x]
        where
            T: Read + Default + Copy,
        {
            #[inline]
            fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
                usize::read(bytes, pos)?;

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
                (&self[..]).write(bytes, pos)
            }
        }

        impl<T> NumBytes for [T; $x]
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
    )*)
}

impl_array!(
     1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40
    41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60
    61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80
    81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100
);
