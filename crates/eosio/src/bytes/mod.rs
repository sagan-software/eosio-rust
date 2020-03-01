//! TODO docs
mod alloc;
mod collections;
mod data_stream;
mod marker;
mod num;
mod option;
mod primitives;

pub use self::data_stream::DataStream;
pub use eosio_macros::{NumBytes, Read, Write};

use ::alloc::{vec, vec::Vec};
use core::fmt;

/// Count the number of bytes a type is expected to use.
pub trait NumBytes {
    /// Count the number of bytes a type is expected to use.
    fn num_bytes(&self) -> usize;
}

/// Read bytes.
pub trait Read: Sized + NumBytes {
    /// Read bytes.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was a problem reading the data.
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError>;

    /// Deserializes a byte array into a data type.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was a problem reading the data.
    fn unpack<T: AsRef<[u8]>>(bytes: T) -> Result<Self, ReadError> {
        Self::read(bytes.as_ref(), &mut 0)
    }
}

/// Error that can be returned when reading bytes.
#[derive(Debug, Clone, Copy)]
pub enum ReadError {
    /// Not enough bytes.
    NotEnoughBytes,
}

impl fmt::Display for ReadError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotEnoughBytes => write!(f, "not enough bytes"),
        }
    }
}

/// Write bytes.
pub trait Write: Sized + NumBytes {
    /// Write bytes.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was a problem writing the data.
    fn write(
        &self,
        bytes: &mut [u8],
        pos: &mut usize,
    ) -> Result<(), WriteError>;

    /// Serializes data into a byte vector.
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was a problem writing the data.
    fn pack(&self) -> Result<Vec<u8>, WriteError> {
        let num_bytes = self.num_bytes();
        let mut bytes = vec![0_u8; num_bytes];
        self.write(&mut bytes, &mut 0)?;
        Ok(bytes)
    }
}

/// Error that can be returned when writing bytes.
#[derive(Debug, Clone, Copy)]
pub enum WriteError {
    /// Not enough space in the vector.
    NotEnoughSpace,
    /// Failed to parse an integer.
    TryFromIntError,
}

impl fmt::Display for WriteError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotEnoughSpace => write!(f, "not enough space"),
            Self::TryFromIntError => write!(f, "failed to parse int"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{vec, NumBytes, Read, Write};
    use ::alloc::{
        string::{String, ToString},
        vec::Vec,
    };

    macro_rules! test_type {
        ($($i:ident, $t:ty, $e:expr)*) => ($(
            #[test]
            #[allow(clippy::float_cmp, clippy::option_unwrap_used)]
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

                let packed = thing.pack().unwrap();
                let unpacked = <$t as Read>::unpack(packed).unwrap();
                assert_eq!(unpacked, thing);
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
        // test_non_zero_u8, NonZeroU8, NonZeroU8::new(1_u8).unwrap()
        // test_non_zero_u16, NonZeroU16, NonZeroU16::new(1_u16).unwrap()
        // test_non_zero_u32, NonZeroU32, NonZeroU32::new(1_u32).unwrap()
        // test_non_zero_u64, NonZeroU64, NonZeroU64::new(1_u64).unwrap()
        // test_non_zero_usize, NonZeroUsize, NonZeroUsize::new(1_usize).unwrap()
        // test_non_zero_i16, NonZeroI16, NonZeroI16::new(-1_i16).unwrap()
        // test_non_zero_i32, NonZeroI32, NonZeroI32::new(-1_i32).unwrap()
        // test_non_zero_i64, NonZeroI64, NonZeroI64::new(-1_i64).unwrap()
        // test_non_zero_isize, NonZeroIsize, NonZeroIsize::new(-1_isize).unwrap()
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
        let bytes = &mut [0_u8; 1000];

        let mut pos = 0;
        1_u8.write(bytes, &mut pos).unwrap();
        assert_eq!(pos, 1);

        1_u16.write(bytes, &mut pos).unwrap();
        assert_eq!(pos, 3);

        1_u32.write(bytes, &mut pos).unwrap();
        assert_eq!(pos, 7);

        1_u64.write(bytes, &mut pos).unwrap();
        assert_eq!(pos, 15);
    }
}
