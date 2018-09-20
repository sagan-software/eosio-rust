#[cfg(all(feature = "alloc", not(feature = "std")))]
use lib::{String, ToString, Vec};

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

impl Writeable for i16 {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let ff = 0xff as i16;
        bytes[0] = (self & ff) as u8;
        bytes[1] = ((self >> 8) & ff) as u8;
        Ok(2)
    }
}

impl Writeable for i32 {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let ff = 0xff as i32;
        bytes[0] = (self & ff) as u8;
        bytes[1] = ((self >> 8) & ff) as u8;
        bytes[2] = ((self >> 16) & ff) as u8;
        bytes[3] = ((self >> 24) & ff) as u8;
        Ok(4)
    }
}

impl Writeable for i64 {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let ff = 0xff as i64;
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
        // TODO: fix this when usize is larger than 1 byte
        (*self as u8).write(bytes)
    }
}

impl Writeable for bool {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let value: u8 = if *self { 1 } else { 0 };
        value.write(bytes)
    }
}

impl<T> Writeable for Option<T>
where
    T: Writeable + Default,
{
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let mut pos = self.is_some().write(bytes)?;
        match self {
            Some(item) => {
                pos += item.write(&mut bytes[pos..])?;
            }
            None => {
                pos += T::default().write(&mut bytes[pos..])?;
            }
        }
        Ok(pos)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
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

#[cfg(any(feature = "std", feature = "alloc"))]
impl Writeable for String {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        self.as_bytes().to_vec().write(bytes)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Writeable for &'a str {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        self.to_string().write(bytes)
    }
}
