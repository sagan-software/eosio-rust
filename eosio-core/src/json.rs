//! TODO docs
use serde::{Deserializer, Serializer};

/// TODO docs
#[inline]
pub fn bool_or_integer<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(BoolOrIntegerVisitor)
}

/// TODO docs
pub struct BoolOrIntegerVisitor;

impl<'de> serde::de::Visitor<'de> for BoolOrIntegerVisitor {
    type Value = bool;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("a bool or integer")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value == 1)
    }
}

/// TODO docs
#[inline]
pub fn bool_to_u8<S>(x: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let num = if *x { 1 } else { 0 };
    s.serialize_u8(num)
}

/// TODO docs
#[inline]
pub fn f64_or_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(F64OrStringVisitor)
}

/// TODO docs
pub struct F64OrStringVisitor;

impl<'de> serde::de::Visitor<'de> for F64OrStringVisitor {
    type Value = f64;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("a number or string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value.parse().map_err(serde::de::Error::custom)
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }
}

/// TODO docs
#[inline]
pub fn u64_or_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(U64OrStringVisitor)
}

/// TODO docs
pub struct U64OrStringVisitor;

impl<'de> serde::de::Visitor<'de> for U64OrStringVisitor {
    type Value = u64;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("a number or string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value.parse().map_err(serde::de::Error::custom)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }
}
