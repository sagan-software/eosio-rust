#[cfg(all(feature = "alloc", not(feature = "std")))]
use lib::{String, Vec};

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
        let num = (u16::from(bytes[0])) | (u16::from(bytes[1]) << 8);
        Ok((num, 2))
    }
}

impl Readable for u32 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 4 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = (u32::from(bytes[0]))
            | (u32::from(bytes[1]) << 8)
            | (u32::from(bytes[2]) << 16)
            | (u32::from(bytes[3]) << 24);
        Ok((num, 4))
    }
}

impl Readable for u64 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 8 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = (u64::from(bytes[0]))
            | (u64::from(bytes[1]) << 8)
            | (u64::from(bytes[2]) << 16)
            | (u64::from(bytes[3]) << 24)
            | (u64::from(bytes[4]) << 32)
            | (u64::from(bytes[5]) << 40)
            | (u64::from(bytes[6]) << 48)
            | (u64::from(bytes[7]) << 56);
        Ok((num, 8))
    }
}

impl Readable for u128 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 16 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = (u128::from(bytes[0]))
            | (u128::from(bytes[1]) << 8)
            | (u128::from(bytes[2]) << 16)
            | (u128::from(bytes[3]) << 24)
            | (u128::from(bytes[4]) << 32)
            | (u128::from(bytes[5]) << 40)
            | (u128::from(bytes[6]) << 48)
            | (u128::from(bytes[7]) << 56)
            | (u128::from(bytes[8]) << 64)
            | (u128::from(bytes[9]) << 72)
            | (u128::from(bytes[10]) << 80)
            | (u128::from(bytes[11]) << 88)
            | (u128::from(bytes[12]) << 96)
            | (u128::from(bytes[13]) << 104)
            | (u128::from(bytes[14]) << 112)
            | (u128::from(bytes[15]) << 120);
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
        let num = (i16::from(bytes[0])) | (i16::from(bytes[1]) << 8);
        Ok((num, 2))
    }
}

impl Readable for i32 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 4 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = (i32::from(bytes[0]))
            | (i32::from(bytes[1]) << 8)
            | (i32::from(bytes[2]) << 16)
            | (i32::from(bytes[3]) << 24);
        Ok((num, 4))
    }
}

impl Readable for i64 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 8 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = (i64::from(bytes[0]))
            | (i64::from(bytes[1]) << 8)
            | (i64::from(bytes[2]) << 16)
            | (i64::from(bytes[3]) << 24)
            | (i64::from(bytes[4]) << 32)
            | (i64::from(bytes[5]) << 40)
            | (i64::from(bytes[6]) << 48)
            | (i64::from(bytes[7]) << 56);
        Ok((num, 8))
    }
}

impl Readable for i128 {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        if bytes.len() < 16 {
            return Err(ReadError::NotEnoughBytes);
        }
        let num = (i128::from(bytes[0]))
            | (i128::from(bytes[1]) << 8)
            | (i128::from(bytes[2]) << 16)
            | (i128::from(bytes[3]) << 24)
            | (i128::from(bytes[4]) << 32)
            | (i128::from(bytes[5]) << 40)
            | (i128::from(bytes[6]) << 48)
            | (i128::from(bytes[7]) << 56)
            | (i128::from(bytes[8]) << 64)
            | (i128::from(bytes[9]) << 72)
            | (i128::from(bytes[10]) << 80)
            | (i128::from(bytes[11]) << 88)
            | (i128::from(bytes[12]) << 96)
            | (i128::from(bytes[13]) << 104)
            | (i128::from(bytes[14]) << 112)
            | (i128::from(bytes[15]) << 120);
        Ok((num, 16))
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

impl<T> Readable for Option<T>
where
    T: Readable,
{
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        let mut pos = 0;
        let (is_some, p) = bool::read(bytes)?;
        pos += p;

        let (item, p) = T::read(bytes)?;
        pos += p;

        let opt = if is_some { Some(item) } else { None };
        Ok((opt, pos))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
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

#[cfg(any(feature = "std", feature = "alloc"))]
impl Readable for String {
    fn read(bytes: &[u8]) -> Result<(Self, usize), ReadError> {
        let (bytes_vec, pos) = Vec::<u8>::read(bytes)?;
        let s = String::from_utf8_lossy(&bytes_vec);
        Ok((s.into_owned(), pos))
    }
}
