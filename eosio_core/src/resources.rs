/// RAM in bytes
pub struct RamBytes(i64);

impl From<i64> for RamBytes {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

/// Net Weight
pub struct NetWeight(i64);

impl From<i64> for NetWeight {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

/// CPU Weight
pub struct CpuWeight(i64);

impl From<i64> for CpuWeight {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
