// https://github.com/EOSIO/eos/blob/master/libraries/chain/abi_serializer.cpp#L65-L103

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
