use serde::{de, Deserialize, Deserializer, Serializer};

#[inline]
pub fn bool_from_u8<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: u8 = Deserialize::deserialize(deserializer)?;
    Ok(s == 1)
}

#[inline]
pub fn bool_to_u8<S>(x: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let num = if *x { 1 } else { 0 };
    s.serialize_u8(num)
}

#[inline]
pub fn f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}

#[inline]
pub fn u64_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}
