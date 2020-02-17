use crate::bytes::{NumBytes, Read, ReadError, Write, WriteError};

/// Variable Length Unsigned Integer. This provides more efficient
/// serialization of 32-bit unsigned int. It serialuzes a 32-bit unsigned
/// integer in as few bytes as possible. `UnsignedInt` is unsigned and uses
/// [VLQ or Base-128 encoding](https://en.wikipedia.org/wiki/Variable-length_quantity)
/// <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/varint.hpp#L15-L237>
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Default)]
pub struct UnsignedInt(u32);

impl From<usize> for UnsignedInt {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    fn from(v: usize) -> Self {
        Self(v as u32)
    }
}

impl From<UnsignedInt> for usize {
    #[must_use]
    fn from(v: UnsignedInt) -> Self {
        v.0 as Self
    }
}

impl From<u32> for UnsignedInt {
    #[must_use]
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl From<u16> for UnsignedInt {
    #[must_use]
    fn from(v: u16) -> Self {
        Self(v.into())
    }
}

impl From<u8> for UnsignedInt {
    #[must_use]
    fn from(v: u8) -> Self {
        Self(v.into())
    }
}

impl NumBytes for UnsignedInt {
    #[inline]
    #[must_use]
    fn num_bytes(&self) -> usize {
        let mut val = u64::from(self.0);
        let mut bytes = 0_usize;
        loop {
            val >>= 7;
            bytes += 1;
            if val == 0 {
                break;
            }
        }
        bytes
    }
}

impl Read for UnsignedInt {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let mut v = 0_u64;
        let mut by = 0_u8;
        loop {
            let b = u8::read(bytes, pos)?;
            v |= u64::from(u32::from(b & 0x7f) << by);
            by += 7;
            if b & 0x80 == 0 {
                break;
            }
        }
        Ok(Self(v as u32))
    }
}

impl Write for UnsignedInt {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        let mut val = u64::from(self.0);
        loop {
            let mut b = (val as u8) & 0x7f;
            val >>= 7;
            b |= ((val > 0) as u8) << 7;
            b.write(bytes, pos)?;
            if val == 0 {
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod unsigned_int_tests {
    use super::UnsignedInt;
    use crate::bytes::{NumBytes, Read, Write};

    macro_rules! write_read_tests {
        ($($i:ident, $v:expr, $n:expr)*) => ($(
            #[test]
            fn $i() {
                let mut bytes = [0_u8; 10];
                let mut write_pos = 0;
                let varint: UnsignedInt = $v.into();
                assert_eq!(varint.num_bytes(), $n);

                varint.write(&mut bytes, &mut write_pos).unwrap();
                assert_eq!(write_pos, $n);
                let mut read_pos = 0;
                let result = UnsignedInt::read(&bytes, &mut read_pos).unwrap();
                assert_eq!(result, varint);
                assert_eq!(read_pos, write_pos);
            }
        )*)
    }

    write_read_tests! {
        read_write_1, 1_u32, 1
        read_write_50, 50_u32, 1
        read_write_u8_min, u8::min_value(), 1
        read_write_u8_max, u8::max_value(), 2
        read_write_u16_min, u16::min_value(), 1
        read_write_u16_max, u16::max_value(), 3
        read_write_u32_min, u32::min_value(), 1
        read_write_u32_max, u32::max_value(), 5
    }
}
