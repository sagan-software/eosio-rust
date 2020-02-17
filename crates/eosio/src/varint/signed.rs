use crate::bytes::{NumBytes, Read, ReadError, Write, WriteError};

/// Variable Length Signed Integer. This provides more efficient serialization
/// of 32-bit signed int. It serializes a 32-bit signed integer in as few bytes
/// as possible. `SignedInt` is signed and uses
/// [Zig-Zag encoding](https://developers.google.com/protocol-buffers/docs/encoding#signed-integers)
/// <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/varint.hpp#L239-L465>
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Default)]
pub struct SignedInt(i32);

impl From<isize> for SignedInt {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    fn from(v: isize) -> Self {
        Self(v as i32)
    }
}

impl From<SignedInt> for isize {
    #[must_use]
    fn from(v: SignedInt) -> Self {
        v.0 as Self
    }
}

impl From<i32> for SignedInt {
    #[must_use]
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl From<SignedInt> for i32 {
    #[must_use]
    fn from(v: SignedInt) -> Self {
        v.0
    }
}

impl From<i16> for SignedInt {
    #[must_use]
    fn from(v: i16) -> Self {
        Self(v.into())
    }
}

impl From<i8> for SignedInt {
    #[must_use]
    fn from(v: i8) -> Self {
        Self(v.into())
    }
}

impl NumBytes for SignedInt {
    #[inline]
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    fn num_bytes(&self) -> usize {
        let mut val = ((self.0 << 1) ^ (self.0 >> 31)) as u32;
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

impl Read for SignedInt {
    #[inline]
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let mut v = 0_u32;
        let mut by = 0_u32;
        loop {
            let b = u8::read(bytes, pos)?;
            v |= u32::from(b & 0x7f) << by;
            by += 7;
            if b & 0x80 == 0 {
                break;
            }
        }
        let value = (v >> 1) ^ ((u64::from(!(v & 1)) + 1_u64) as u32);
        Ok(Self(value as i32))
    }
}

impl Write for SignedInt {
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError> {
        let mut val = ((self.0 << 1) ^ (self.0 >> 31)) as u32;
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
mod signed_int_tests {
    use super::SignedInt;
    use crate::bytes::{NumBytes, Read, Write};

    macro_rules! write_read_tests {
        ($($i:ident, $v:expr, $n:expr)*) => ($(
            #[test]
            fn $i() {
                let mut bytes = [0_u8; 10];
                let mut write_pos = 0;
                let varint: SignedInt = $v.into();
                assert_eq!(varint.num_bytes(), $n);

                varint.write(&mut bytes, &mut write_pos).unwrap();
                assert_eq!(write_pos, $n);
                let mut read_pos = 0;
                let result = SignedInt::read(&bytes, &mut read_pos).unwrap();
                assert_eq!(result, varint);
                assert_eq!(read_pos, write_pos);
            }
        )*)
    }

    write_read_tests! {
        read_write_1, 1_i32, 1
        read_write_1_neg, -1_i32, 1
        read_write_50, 50_i32, 1
        read_write_50_neg, -50_i32, 1
        read_write_i8_min, i8::min_value(), 2
        read_write_i8_zero, 0_i8, 1
        read_write_i8_max, i8::max_value(), 2
        read_write_i16_min, i16::min_value(), 3
        read_write_i16_zero, 0_i16, 1
        read_write_i16_max, i16::max_value(), 3
        read_write_i32_min, i32::min_value(), 5
        read_write_i32_zero, 0_i32, 1
        read_write_i32_max, i32::max_value(), 5
    }
}
