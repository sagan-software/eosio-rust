use crate::lib::TryInto;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use lib::{String, ToString, Vec};

#[derive(Debug, Clone, Copy)]
pub enum ReadError {
    NotEnoughBytes,
}

pub trait Read: Sized {
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError>;
}

#[derive(Debug, Clone, Copy)]
pub enum WriteError {
    NotEnoughSpace,
    TryFromIntError,
}

pub trait Write: Sized {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError>;
}

macro_rules! impl_num {
    ($($t:ty, $s:expr)*) => ($(
        impl Read for $t {
            fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
                let width: usize = $s;
                let end_pos = pos + width;
                if bytes.len() < end_pos {
                    return Err(ReadError::NotEnoughBytes);
                }

                let mut num = <$t as From<u8>>::from(0 as u8);
                for i in 0..width {
                    match bytes.get(pos + i) {
                        Some(b) => {
                            let shift = <$t as From<u8>>::from(i as u8) * <$t as From<u8>>::from(8u8);
                            num |= <$t as From<u8>>::from(*b) << shift;
                        }
                        None => return Err(ReadError::NotEnoughBytes),
                    }
                }
                Ok((num, end_pos))
            }
        }

        impl Write for $t
        {
            fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
                let width: usize = $s;
                let end_pos = pos + width;
                if bytes.len() < end_pos {
                    return Err(WriteError::NotEnoughSpace);
                }

                for i in 0..width {
                    match bytes.get_mut(pos + i) {
                        Some(byte) => {
                            let ff = <$t as From<u8>>::from(0xff);
                            let shift = <$t as From<u8>>::from(i as u8) * <$t as From<u8>>::from(8u8);
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
    )*)
}

impl_num!(
    u8, 1
    u16, 2
    i16, 2
    u32, 4
    i32, 4
    u64, 8
    i64, 8
); // TODO i8 u128 i128

impl Read for f32 {
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
        let (bits, pos) = u32::read(bytes, pos)?;
        let num = f32::from_bits(bits);
        Ok((num, pos))
    }
}

impl Write for f32 {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let bits = f32::to_bits(*self);
        bits.write(bytes, pos)
    }
}

impl Read for f64 {
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
        let (bits, pos) = u64::read(bytes, pos)?;
        let num = f64::from_bits(bits);
        Ok((num, pos))
    }
}

impl Write for f64 {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let bits = f64::to_bits(*self);
        bits.write(bytes, pos)
    }
}

impl Read for bool {
    fn read(bytes: &[u8], offset: usize) -> Result<(Self, usize), ReadError> {
        u8::read(bytes, offset).map(|(v, c)| (v == 1, c))
    }
}

impl Write for bool {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let value: u8 = if *self { 1 } else { 0 };
        value.write(bytes, pos)
    }
}

impl Read for usize {
    fn read(bytes: &[u8], offset: usize) -> Result<(Self, usize), ReadError> {
        // TODO: fix this. usize isn't always u8?
        u8::read(bytes, offset).map(|(v, c)| (v as usize, c))
    }
}

impl Write for usize {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        // TODO: fix this when usize is larger than 1 byte?
        (*self as u8).write(bytes, pos)
    }
}

impl<T> Read for Option<T>
where
    T: Read,
{
    fn read(bytes: &[u8], offset: usize) -> Result<(Self, usize), ReadError> {
        let (is_some, offset) = bool::read(bytes, offset)?;
        let (item, offset) = T::read(bytes, offset)?;
        let opt = if is_some { Some(item) } else { None };
        Ok((opt, offset))
    }
}

impl<T> Write for Option<T>
where
    T: Write + Default,
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

impl<'a, T> Write for &'a [T]
where
    T: Write,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let mut pos = self.len().write(bytes, pos)?;
        for item in self.iter() {
            pos = item.write(bytes, pos)?;
        }
        Ok(pos)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> Read for Vec<T>
where
    T: Read + Default + Clone,
{
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
        let (capacity, pos) = usize::read(bytes, pos)?;

        let mut results = Vec::new();
        results.resize(capacity, T::default());

        let mut pos = pos;
        for item in results.iter_mut() {
            let (r, p) = T::read(bytes, pos)?;
            *item = r;
            pos = p;
        }

        Ok((results, pos))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> Write for Vec<T>
where
    T: Write,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let mut pos = self.len().write(bytes, pos)?;
        for item in self.iter() {
            pos = item.write(bytes, pos)?;
        }
        Ok(pos)
    }
}

macro_rules! impl_array {
    ($($x:expr)*) => ($(
        impl<T> Read for [T; $x]
        where
            T: Read + Default + Copy,
        {
            fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
                let (_, pos) = usize::read(bytes, pos)?;

                let mut items = [T::default(); $x];
                let mut pos = pos;
                for item in items.iter_mut() {
                    let (v, p) = T::read(bytes, pos)?;
                    *item = v;
                    pos = p;
                }

                Ok((items, pos))
            }
        }

        impl<T> Write for [T; $x]
        where
            T: Write,
        {
            fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
                (&self[..]).write(bytes, pos)
            }
        }
    )*)
}

impl_array!(
     1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40
    41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60
    61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80
    81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100
);

#[cfg(any(feature = "std", feature = "alloc"))]
impl Read for String {
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
        // TODO: may need to read this as a cstr
        let (bytes, pos) = Vec::<u8>::read(bytes, pos)?;
        let s = String::from_utf8_lossy(&bytes);
        Ok((s.into_owned(), pos))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Write for String {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        self.as_bytes().write(bytes, pos)
    }
}

impl<'a> Write for &'a str {
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        self.as_bytes().write(bytes, pos)
    }
}

impl<A, B> Read for (A, B)
where
    A: Read,
    B: Read,
{
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
        let (a, pos) = A::read(bytes, pos)?;
        let (b, pos) = B::read(bytes, pos)?;
        Ok(((a, b), pos))
    }
}

impl<A, B> Write for (A, B)
where
    A: Write,
    B: Write,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let pos = self.0.write(bytes, pos)?;
        let pos = self.1.write(bytes, pos)?;
        Ok(pos)
    }
}

impl<A, B, C> Read for (A, B, C)
where
    A: Read,
    B: Read,
    C: Read,
{
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
        let (a, pos) = A::read(bytes, pos)?;
        let (b, pos) = B::read(bytes, pos)?;
        let (c, pos) = C::read(bytes, pos)?;
        Ok(((a, b, c), pos))
    }
}

impl<A, B, C> Write for (A, B, C)
where
    A: Write,
    B: Write,
    C: Write,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let pos = self.0.write(bytes, pos)?;
        let pos = self.1.write(bytes, pos)?;
        let pos = self.2.write(bytes, pos)?;
        Ok(pos)
    }
}

impl<A, B, C, D> Read for (A, B, C, D)
where
    A: Read,
    B: Read,
    C: Read,
    D: Read,
{
    fn read(bytes: &[u8], pos: usize) -> Result<(Self, usize), ReadError> {
        let (a, pos) = A::read(bytes, pos)?;
        let (b, pos) = B::read(bytes, pos)?;
        let (c, pos) = C::read(bytes, pos)?;
        let (d, pos) = D::read(bytes, pos)?;
        Ok(((a, b, c, d), pos))
    }
}

impl<A, B, C, D> Write for (A, B, C, D)
where
    A: Write,
    B: Write,
    C: Write,
    D: Write,
{
    fn write(&self, bytes: &mut [u8], pos: usize) -> Result<usize, WriteError> {
        let pos = self.0.write(bytes, pos)?;
        let pos = self.1.write(bytes, pos)?;
        let pos = self.2.write(bytes, pos)?;
        let pos = self.3.write(bytes, pos)?;
        Ok(pos)
    }
}
