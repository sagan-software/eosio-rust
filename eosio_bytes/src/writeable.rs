use fixed_size::FixedSize;
use lib::{BitAnd, Mul, Shr, TryInto};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use lib::{String, ToString, Vec};

#[derive(Debug)]
pub enum WriteError {
    NotEnoughSpace,
    TryFromIntError,
}

pub trait Writeable: Sized {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError>;
}

impl<T> Writeable for T
where
    T: FixedSize
        + From<u8>
        + TryInto<u8>
        + BitAnd<Output = T>
        + Shr<Output = T>
        + Mul<Output = T>
        + Copy,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let width = T::size();
        let end_pos = pos + width;
        if bytes.len() < end_pos {
            return Err(WriteError::NotEnoughSpace);
        }

        for i in 0..width {
            match bytes.get_mut(pos + i) {
                Some(byte) => {
                    let ff = T::from(0xff);
                    let shift = T::from(i as u8) * T::from(8u8);
                    let result = ((*self >> shift) & ff).try_into();
                    match result {
                        Ok(b) => *byte = b,
                        Err(_) => return Err(WriteError::TryFromIntError),
                    }
                }
                None => return Err(WriteError::NotEnoughSpace),
            }
        }

        Ok(end_pos)
    }
}

impl Writeable for usize {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        // TODO: fix this when usize is larger than 1 byte
        (*self as u8).write(bytes, pos)
    }
}

impl Writeable for bool {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let value: u8 = if *self { 1 } else { 0 };
        value.write(bytes, pos)
    }
}

impl<T> Writeable for Option<T>
where
    T: Writeable + Default,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let pos = self.is_some().write(bytes, pos)?;
        let pos = match self {
            Some(item) => item.write(bytes, pos)?,
            None => T::default().write(bytes, pos)?,
        };
        Ok(pos)
    }
}

impl<'a, T> Writeable for &'a [T]
where
    T: Writeable,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let mut pos = self.len().write(bytes, pos)?;
        for item in self.iter() {
            pos = item.write(bytes, pos)?;
        }
        Ok(pos)
    }
}

macro_rules! impl_fixed_array {
    ($x:expr) => {
        impl<T> Writeable for [T; $x]
        where
            T: Writeable,
        {
            fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
                (&self).write(bytes, pos)
            }
        }
    };
}

impl_fixed_array!(1);
impl_fixed_array!(2);
impl_fixed_array!(3);
impl_fixed_array!(4);
impl_fixed_array!(5);
impl_fixed_array!(6);
impl_fixed_array!(7);
impl_fixed_array!(8);
impl_fixed_array!(9);

// #[cfg(any(feature = "std", feature = "alloc"))]
// impl<T> Writeable for Vec<T>
// where
//     T: Writeable,
// {
//     fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
//         let pos = self.len().write(bytes)?;
//         for item in self.iter() {
//             let pos = item.write(&mut bytes[pos..])?;
//         }
//         Ok(pos)
//     }
// }

#[cfg(any(feature = "std", feature = "alloc"))]
impl Writeable for String {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        // let s = ::eosio_sys::CString::new(self.clone()).unwrap();
        // s.into_bytes_with_nul().write(bytes)
        self.as_bytes().write(bytes, pos)
    }
}

// impl<'a> Writeable for &'a str {
//     fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
//         let pos = self.len().write(bytes)?;
//         let pos = pos + self.as_bytes().write(&mut bytes[pos..])?;
//         Ok(pos)
//     }
// }

// impl<A, B> Writeable for (A, B)
// where
//     A: Writeable,
//     B: Writeable,
// {
//     fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
//         let pos = self.0.write(bytes)?;
//         let pos = pos + self.1.write(&mut bytes[pos..])?;
//         Ok(pos)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_pos() {
        let mut bytes = &mut [0u8; 1000];
        let pos = 0;

        let pos = 1u8.write(bytes, pos).unwrap();
        assert_eq!(pos, 1);

        let pos = 1u16.write(bytes, pos).unwrap();
        assert_eq!(pos, 3);

        let pos = 1u32.write(bytes, pos).unwrap();
        assert_eq!(pos, 7);

        let pos = 1u64.write(bytes, pos).unwrap();
        assert_eq!(pos, 15);
    }
}
