pub trait FixedSize: Sized {
    fn size() -> usize;
}

impl FixedSize for u8 {
    fn size() -> usize {
        1
    }
}

impl FixedSize for i8 {
    fn size() -> usize {
        1
    }
}

impl FixedSize for u16 {
    fn size() -> usize {
        2
    }
}

impl FixedSize for i16 {
    fn size() -> usize {
        2
    }
}

impl FixedSize for u32 {
    fn size() -> usize {
        4
    }
}

impl FixedSize for i32 {
    fn size() -> usize {
        4
    }
}

impl FixedSize for u64 {
    fn size() -> usize {
        8
    }
}

impl FixedSize for i64 {
    fn size() -> usize {
        8
    }
}

impl FixedSize for u128 {
    fn size() -> usize {
        16
    }
}

impl FixedSize for i128 {
    fn size() -> usize {
        16
    }
}
