#[cfg(all(feature = "alloc", not(feature = "std")))]
use lib::{String, ToString, Vec};

#[derive(Debug, Clone, Copy)]
pub enum ReadError {
    NotEnoughBytes,
}

pub trait Read: Sized {
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError>;
}

#[derive(Debug, Clone, Copy)]
pub enum WriteError {
    NotEnoughSpace,
    TryFromIntError,
}

pub trait Write: Sized {
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError>;
}

pub trait NumBytes {
    fn num_bytes(&self) -> usize;
}

macro_rules! impl_num {
    ($($t:ty, $s:expr)*) => ($(
        impl Read for $t {
            #[inline]
            fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
                let width: usize = $s;

                let mut num = <$t as From<u8>>::from(0_u8);
                for i in 0..width {
                    match bytes.get(*pos) {
                        Some(b) => {
                            let shift = <$t as From<u8>>::from(i as u8).saturating_mul(<$t as From<u8>>::from(8_u8));
                            num |= <$t as From<u8>>::from(*b) << shift;
                        }
                        None => return Err(ReadError::NotEnoughBytes),
                    }
                    *pos = pos.saturating_add(1);
                }
                Ok(num)
            }
        }

        impl Write for $t
        {
            #[inline]
            fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
                let width: usize = $s;

                for i in 0..width {
                    match bytes.get_mut(*pos) {
                        Some(byte) => {
                            let ff = <$t as From<u8>>::from(0xff);
                            let shift = <$t as From<u8>>::from(i as u8).saturating_mul(<$t as From<u8>>::from(8_u8));
                            // TODO when try_into is stablized:
                            // let result = ((*self >> shift) & ff).try_into();
                            // match result {
                            //     Ok(b) => *byte = b,
                            //     Err(_) => return Err(WriteError::TryFromIntError),
                            // }
                            *byte = ((*self >> shift) & ff) as u8;
                        }
                        None => return Err(WriteError::NotEnoughSpace),
                    }
                    *pos = pos.saturating_add(1);
                }

                Ok(())
            }
        }

        impl NumBytes for $t
        {
            #[inline]
            fn num_bytes(&self) -> usize {
                $s
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
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let bits = u32::read(bytes, pos)?;
        let num = Self::from_bits(bits);
        Ok(num)
    }
}

impl Write for f32 {
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.to_bits().write(bytes, pos)
    }
}

impl NumBytes for f32 {
    #[inline]
    fn num_bytes(&self) -> usize {
        4
    }
}

impl Read for f64 {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let bits = u64::read(bytes, pos)?;
        let num = Self::from_bits(bits);
        Ok(num)
    }
}

impl Write for f64 {
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.to_bits().write(bytes, pos)
    }
}

impl NumBytes for f64 {
    #[inline]
    fn num_bytes(&self) -> usize {
        8
    }
}

impl Read for bool {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        u8::read(bytes, pos).map(|v| v == 1)
    }
}

impl Write for bool {
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        let value: u8 = if *self { 1 } else { 0 };
        value.write(bytes, pos)
    }
}

impl NumBytes for bool {
    #[inline]
    fn num_bytes(&self) -> usize {
        1
    }
}

impl Read for usize {
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        // TODO: fix this. usize isn't always u8?
        u8::read(bytes, pos).map(|v| v as Self)
    }
}

impl Write for usize {
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        // TODO: fix this when usize is larger than 1 byte?
        (*self as u8).write(bytes, pos)
    }
}

impl NumBytes for usize {
    #[inline]
    fn num_bytes(&self) -> usize {
        1
    }
}

impl<T> Read for Option<T>
where
    T: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let is_some = bool::read(bytes, pos)?;
        let item = T::read(bytes, pos)?;
        let opt = if is_some { Some(item) } else { None };
        Ok(opt)
    }
}

impl<T> Write for Option<T>
where
    T: Write + Default,
{
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.is_some().write(bytes, pos)?;
        match self {
            Some(item) => item.write(bytes, pos)?,
            None => T::default().write(bytes, pos)?,
        };
        Ok(())
    }
}

impl<'a, T> Write for &'a [T]
where
    T: Write,
{
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.len().write(bytes, pos)?;
        for item in self.iter() {
            item.write(bytes, pos)?;
        }
        Ok(())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> Read for Vec<T>
where
    T: Read + Default + Clone,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let capacity = usize::read(bytes, pos)?;

        let mut results = Self::new();
        results.resize(capacity, T::default());

        for item in &mut results {
            let r = T::read(bytes, pos)?;
            *item = r;
        }

        Ok(results)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> NumBytes for Vec<T>
where
    T: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        let mut count = 1;
        for item in self.iter() {
            count += item.num_bytes();
        }
        count
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> Write for Vec<T>
where
    T: Write,
{
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.len().write(bytes, pos)?;
        for item in self.iter() {
            item.write(bytes, pos)?;
        }
        Ok(())
    }
}

macro_rules! impl_array {
    ($($x:expr)*) => ($(
        impl<T> Read for [T; $x]
        where
            T: Read + Default + Copy,
        {
            #[inline]
            fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
                usize::read(bytes, pos)?;

                let mut items = [T::default(); $x];
                for item in items.iter_mut() {
                    let v = T::read(bytes, pos)?;
                    *item = v;
                }

                Ok(items)
            }
        }

        impl<T> Write for [T; $x]
        where
            T: Write,
        {
            #[inline]
            fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
                (&self[..]).write(bytes, pos)
            }
        }

        impl<T> NumBytes for [T; $x]
        where
            T: NumBytes,
        {
            #[inline]
            fn num_bytes(&self) -> usize {
                let mut count = 1;
                for item in self.iter() {
                    count += item.num_bytes();
                }
                count
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
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        // TODO: may need to read this as a cstr
        let utf8 = Vec::<u8>::read(bytes, pos)?;
        let s = Self::from_utf8_lossy(&utf8);
        Ok(s.into_owned())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Write for String {
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.as_bytes().write(bytes, pos)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl NumBytes for String {
    #[inline]
    fn num_bytes(&self) -> usize {
        self.len().saturating_add(1) // TODO: utf16?
    }
}

impl<'a> Write for &'a str {
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.as_bytes().write(bytes, pos)
    }
}

impl<'a> NumBytes for &'a str {
    #[inline]
    fn num_bytes(&self) -> usize {
        self.len().saturating_add(1) // TODO: utf16?
    }
}

impl<A, B> Read for (A, B)
where
    A: Read,
    B: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let a = A::read(bytes, pos)?;
        let b = B::read(bytes, pos)?;
        Ok((a, b))
    }
}

impl<A, B> NumBytes for (A, B)
where
    A: NumBytes,
    B: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        self.0.num_bytes().saturating_add(self.1.num_bytes())
    }
}

impl<A, B> Write for (A, B)
where
    A: Write,
    B: Write,
{
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.0.write(bytes, pos)?;
        self.1.write(bytes, pos)?;
        Ok(())
    }
}

impl<A, B, C> Read for (A, B, C)
where
    A: Read,
    B: Read,
    C: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let a = A::read(bytes, pos)?;
        let b = B::read(bytes, pos)?;
        let c = C::read(bytes, pos)?;
        Ok((a, b, c))
    }
}

impl<A, B, C> NumBytes for (A, B, C)
where
    A: NumBytes,
    B: NumBytes,
    C: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        self.0
            .num_bytes()
            .saturating_add(self.1.num_bytes())
            .saturating_add(self.2.num_bytes())
    }
}

impl<A, B, C> Write for (A, B, C)
where
    A: Write,
    B: Write,
    C: Write,
{
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.0.write(bytes, pos)?;
        self.1.write(bytes, pos)?;
        self.2.write(bytes, pos)?;
        Ok(())
    }
}

impl<A, B, C, D> Read for (A, B, C, D)
where
    A: Read,
    B: Read,
    C: Read,
    D: Read,
{
    #[inline]
    fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
        let a = A::read(bytes, pos)?;
        let b = B::read(bytes, pos)?;
        let c = C::read(bytes, pos)?;
        let d = D::read(bytes, pos)?;
        Ok((a, b, c, d))
    }
}

impl<A, B, C, D> NumBytes for (A, B, C, D)
where
    A: NumBytes,
    B: NumBytes,
    C: NumBytes,
    D: NumBytes,
{
    #[inline]
    fn num_bytes(&self) -> usize {
        self.0
            .num_bytes()
            .saturating_add(self.1.num_bytes())
            .saturating_add(self.2.num_bytes())
            .saturating_add(self.3.num_bytes())
    }
}

impl<A, B, C, D> Write for (A, B, C, D)
where
    A: Write,
    B: Write,
    C: Write,
    D: Write,
{
    #[inline]
    fn write(&self, bytes: &mut [u8], pos: &mut usize) -> Result<(), WriteError> {
        self.0.write(bytes, pos)?;
        self.1.write(bytes, pos)?;
        self.2.write(bytes, pos)?;
        self.3.write(bytes, pos)?;
        Ok(())
    }
}
