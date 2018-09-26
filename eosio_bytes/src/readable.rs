use fixed_size::FixedSize;
use lib::{BitOr, Mul, Shl};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use lib::{String, Vec};

#[derive(Debug)]
pub enum ReadError {
    NotEnoughBytes,
}

pub trait Readable: Sized {
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError>;
}

impl<T> Readable for T
where
    T: FixedSize + From<u8> + BitOr<Output = T> + Shl<Output = T> + Mul<Output = T>,
{
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
        let width = T::size();
        let end_pos = pos + width;
        if bytes.len() < end_pos {
            return Err(ReadError::NotEnoughBytes);
        }

        let mut num = T::from(0 as u8);
        for i in 0..width {
            match bytes.get(pos + i) {
                Some(b) => {
                    let shift = T::from(i as u8) * T::from(8 as u8);
                    num = num | (T::from(*b) << shift);
                }
                None => return Err(ReadError::NotEnoughBytes),
            }
        }
        Ok((num, end_pos))
    }
}

impl Readable for bool {
    fn read(bytes: &[u8], offset: usize) -> Result<(Self, usize), ReadError> {
        u8::read(bytes, offset).map(|(v, c)| (v == 1, c))
    }
}

impl Readable for usize {
    fn read(bytes: &[u8], offset: usize) -> Result<(Self, usize), ReadError> {
        // TODO: fix this. usize isn't always u8?
        u8::read(bytes, offset).map(|(v, c)| (v as usize, c))
    }
}

impl<T> Readable for Option<T>
where
    T: Readable,
{
    fn read(bytes: &[u8], offset: usize) -> Result<(Self, usize), ReadError> {
        let (is_some, offset) = bool::read(bytes, offset)?;
        let (item, offset) = T::read(bytes, offset)?;
        let opt = if is_some { Some(item) } else { None };
        Ok((opt, offset))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> Readable for Vec<T>
where
    T: Readable,
{
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
        let (capacity, pos) = usize::read(bytes, pos)?;

        let mut results = Vec::with_capacity(capacity);
        let mut pos = pos;
        for _i in 0..capacity {
            let (r, p) = T::read(bytes, pos)?;
            results.push(r);
            pos = p;
        }

        Ok((results, pos))
    }
}

impl<T> Readable for [T; 9]
where
    T: Readable + Default + Copy,
{
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
        let (capacity, pos) = usize::read(bytes, pos)?;

        let mut items = [T::default(); 9];
        let mut pos = pos;
        for item in items.iter_mut() {
            let (v, p) = T::read(bytes, pos)?;
            *item = v;
            pos = p;
        }

        Ok((items, pos))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Readable for String {
    fn read(bytes: &[u8], offset: usize) -> Result<(Self, usize), ReadError> {
        // TODO: may need to read this as a cstr
        let (bytes_vec, pos) = Vec::<u8>::read(bytes, offset)?;
        let s = ::eosio_sys::CStr::from_bytes_with_nul(&bytes_vec);
        let s = match s {
            Ok(s) => s.to_string_lossy(),
            Err(_) => String::from_utf8_lossy(&bytes_vec),
        };
        Ok((s.into_owned(), pos))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_pos() {
        let bytes = &[
            10, 9, 0, 1, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 20, 4, 3, 2, 1, 1, 1, 1, 1,
        ];
        let pos = 0;

        let (_, pos) = u8::read(bytes, pos).unwrap();
        assert_eq!(pos, 1);

        let (_, pos) = u8::read(bytes, pos).unwrap();
        assert_eq!(pos, 2);

        let (_, pos) = u16::read(bytes, pos).unwrap();
        assert_eq!(pos, 4);

        let (_, pos) = u32::read(bytes, pos).unwrap();
        assert_eq!(pos, 8);

        let (_, pos) = u64::read(bytes, pos).unwrap();
        assert_eq!(pos, 16);

        let (_, pos) = u64::read(bytes, pos).unwrap();
        assert_eq!(pos, 24);

        let (_, pos) = u64::read(bytes, 2).unwrap();
        assert_eq!(pos, 10);

        let (_, pos) = u64::read(bytes, 10).unwrap();
        assert_eq!(pos, 18);
    }
}
