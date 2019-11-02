use super::{NumBytes, Read, ReadError, Write, WriteError};
use core::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize,
};

macro_rules! impl_non_zero_nums {
    ($($t:ty, $non_zero:ty)*) => ($(
        impl NumBytes for $non_zero {
            #[inline]
            #[must_use]
            fn num_bytes(&self) -> usize {
                self.get().num_bytes()
            }
        }

        impl Read for $non_zero {
            #[inline]
            fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
                let n = <$t as Read>::read(bytes, pos)?;
                // TODO don't panic
                let num = Self::new(n).expect("got zero for non-zero number");
                Ok(num)
            }
        }

        impl Write for $non_zero {
            #[inline]
            fn write(
                &self,
                bytes: &mut [u8],
                pos: &mut usize,
            ) -> Result<(), WriteError> {
                self.get().write(bytes, pos)
            }
        }
    )*)
}

impl_non_zero_nums! {
    i16, NonZeroI16
    i32, NonZeroI32
    i64, NonZeroI64
    isize, NonZeroIsize
    u16, NonZeroU16
    u32, NonZeroU32
    u64, NonZeroU64
    u8, NonZeroU8
    usize, NonZeroUsize
} // TODO i8 u128 i128
