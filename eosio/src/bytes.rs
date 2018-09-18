// https://github.com/EOSIO/eos/blob/master/libraries/chain/abi_serializer.cpp#L65-L103

use alloc::prelude::Vec;
use eosio_types::Name;

#[derive(Debug)]
pub enum ReadError {
    NotEnoughBytes,
}

pub trait Readable: Sized {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError>;
}

impl Readable for u8 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.is_empty() {
            return Err(ReadError::NotEnoughBytes);
        }
        Ok((bytes[0], 1))
    }
}

impl Readable for u16 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 2 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = ((bytes[0] as u16) << 0) | ((bytes[1] as u16) << 8);
        Ok((num, 2))
    }
}

impl Readable for u32 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 4 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = ((bytes[0] as u32) << 0)
            | ((bytes[1] as u32) << 8)
            | ((bytes[2] as u32) << 16)
            | ((bytes[3] as u32) << 24);
        Ok((num, 4))
    }
}

impl Readable for u64 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 8 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = ((bytes[0] as u64) << 0)
            | ((bytes[1] as u64) << 8)
            | ((bytes[2] as u64) << 16)
            | ((bytes[3] as u64) << 24)
            | ((bytes[4] as u64) << 32)
            | ((bytes[5] as u64) << 40)
            | ((bytes[6] as u64) << 48)
            | ((bytes[7] as u64) << 56);
        Ok((num, 8))
    }
}

impl Readable for u128 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 16 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = ((bytes[0] as u128) << 0)
            | ((bytes[1] as u128) << 8)
            | ((bytes[2] as u128) << 16)
            | ((bytes[3] as u128) << 24)
            | ((bytes[4] as u128) << 32)
            | ((bytes[5] as u128) << 40)
            | ((bytes[6] as u128) << 48)
            | ((bytes[7] as u128) << 56)
            | ((bytes[8] as u128) << 64)
            | ((bytes[9] as u128) << 72)
            | ((bytes[10] as u128) << 80)
            | ((bytes[11] as u128) << 88)
            | ((bytes[12] as u128) << 96)
            | ((bytes[13] as u128) << 104)
            | ((bytes[14] as u128) << 112)
            | ((bytes[15] as u128) << 120);
        Ok((num, 16))
    }
}

impl Readable for i8 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.is_empty() {
            return Err(ReadError::NotEnoughBytes);
        }
        Ok((bytes[0] as i8, 1))
    }
}

impl Readable for i16 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 2 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = ((bytes[0] as i16) << 0) | ((bytes[1] as i16) << 8);
        Ok((num, 2))
    }
}

impl Readable for i32 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 4 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = ((bytes[0] as i32) << 0)
            | ((bytes[1] as i32) << 8)
            | ((bytes[2] as i32) << 16)
            | ((bytes[3] as i32) << 24);
        Ok((num, 4))
    }
}

impl Readable for i64 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 8 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = ((bytes[0] as i64) << 0)
            | ((bytes[1] as i64) << 8)
            | ((bytes[2] as i64) << 16)
            | ((bytes[3] as i64) << 24)
            | ((bytes[4] as i64) << 32)
            | ((bytes[5] as i64) << 40)
            | ((bytes[6] as i64) << 48)
            | ((bytes[7] as i64) << 56);
        Ok((num, 8))
    }
}

impl Readable for i128 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 16 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = ((bytes[0] as i128) << 0)
            | ((bytes[1] as i128) << 8)
            | ((bytes[2] as i128) << 16)
            | ((bytes[3] as i128) << 24)
            | ((bytes[4] as i128) << 32)
            | ((bytes[5] as i128) << 40)
            | ((bytes[6] as i128) << 48)
            | ((bytes[7] as i128) << 56)
            | ((bytes[8] as i128) << 64)
            | ((bytes[9] as i128) << 72)
            | ((bytes[10] as i128) << 80)
            | ((bytes[11] as i128) << 88)
            | ((bytes[12] as i128) << 96)
            | ((bytes[13] as i128) << 104)
            | ((bytes[14] as i128) << 112)
            | ((bytes[15] as i128) << 120);
        Ok((num, 16))
    }
}

impl Readable for ::types::Name {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        u64::read(bytes).map(|(v, c)| (Name::new(v), c))
    }
}

impl Readable for bool {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        u8::read(bytes).map(|(v, c)| (v == 1, c))
    }
}

impl Readable for usize {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        // TODO: fix this. usize isn't always u8
        u8::read(bytes).map(|(v, c)| (v as usize, c))
    }
}

impl<T> Readable for Vec<T>
where
    T: Readable,
{
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        let mut pos = 0;
        let (capacity, p) = usize::read(bytes)?;
        pos += p;

        let mut results = Vec::new();
        for _i in 0..capacity {
            let (r, p) = T::read(&bytes[pos..])?;
            results.push(r);
            pos += p;
        }

        Ok((results, pos))
    }
}

// string
// vector
// f32
// f64
// time_point
// time_point_sec
// block_timestamp_type
// bytes
// checksum160
// checksum256
// checksum512
// public_key
// signature
// symbol
// symbol_code
// asset
// extended_asset

#[derive(Debug)]
pub enum WriteError {
    NotEnoughSpace,
}

pub trait Writeable: Sized {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError>;
}

impl Writeable for u8 {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let ff = 0xff as u8;
        bytes[0] = (self & ff) as u8;
        Ok(1)
    }
}

impl Writeable for u16 {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let ff = 0xff as u16;
        bytes[0] = (self & ff) as u8;
        bytes[1] = ((self >> 8) & ff) as u8;
        Ok(2)
    }
}

impl Writeable for u32 {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let ff = 0xff as u32;
        bytes[0] = (self & ff) as u8;
        bytes[1] = ((self >> 8) & ff) as u8;
        bytes[2] = ((self >> 16) & ff) as u8;
        bytes[3] = ((self >> 24) & ff) as u8;
        Ok(4)
    }
}

impl Writeable for u64 {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let ff = 0xff as u64;
        bytes[0] = (self & ff) as u8;
        bytes[1] = ((self >> 8) & ff) as u8;
        bytes[2] = ((self >> 16) & ff) as u8;
        bytes[3] = ((self >> 24) & ff) as u8;
        bytes[4] = ((self >> 32) & ff) as u8;
        bytes[5] = ((self >> 40) & ff) as u8;
        bytes[6] = ((self >> 48) & ff) as u8;
        bytes[7] = ((self >> 56) & ff) as u8;
        Ok(8)
    }
}

impl Writeable for usize {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        // TODO: fix this when usize is larger than 2 bytes
        (*self as u8).write(bytes)
    }
}

impl Writeable for ::types::Name {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        self.as_u64().write(bytes)
    }
}

impl Writeable for bool {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let value: u8 = if *self { 1 } else { 0 };
        value.write(bytes)
    }
}

impl<T> Writeable for Vec<T>
where
    T: Writeable,
{
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let mut pos = self.len().write(bytes)?;
        for item in self.iter() {
            pos += item.write(&mut bytes[pos..])?;
        }
        Ok(pos)
    }
}
