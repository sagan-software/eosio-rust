use crate::{AccountName, Symbol};
use eosio_bytes::{NumBytes, Read, Write};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write, NumBytes)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Asset {
    pub amount: i64,
    pub symbol: Symbol,
}

impl Asset {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.symbol.is_valid()
    }
}

impl fmt::Display for Asset {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let precision = self.symbol.precision() as usize;
        let amount = (self.amount as f64) / 10_f64.powf(precision as f64);
        let symbol_code = self.symbol.code();
        write!(f, "{:.*} {}", precision, amount, symbol_code)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseAssetError {
    BadChar(char),
    BadPrecision,
}

impl std::str::FromStr for Asset {
    type Err = ParseAssetError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let chars = s.chars();
        let mut index = 0;
        // Find numbers
        for c in chars {
            if index == 0 {
                if '1' <= c && c <= '9' {
                    index += 1;
                    continue;
                } else {
                    return Err(ParseAssetError::BadChar(c));
                }
            }

            if '0' <= c && c <= '9' {
                index += 1;
            }
        }

        // Parse numbers
        // for c in chars {
        //     if c < 'A' || c > 'Z' {
        //         // return Err(ParseSymbolError::BadChar(c));
        //     } else {
        //         result |= (c as u64) << (8 * (i + 1));
        //     }
        // }
        // Look for dot
        // Look for decimal fractions
        // Skip 1 space
        // Look for symbol code
        Ok(Self {
            amount: 0,
            symbol: 0.into(),
        })
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for Asset {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eosio_numstr_macros::{n, s};

    macro_rules! test_to_string {
        ($($name:ident, $amount:expr, $symbol:expr, $expected:expr)*) => ($(
            #[test]
            fn $name() {
                let asset = Asset {
                    amount: $amount,
                    symbol: $symbol.into(),
                };
                assert_eq!(asset.to_string(), $expected);
            }
        )*)
    }

    test_to_string! {
        to_string, 1_0000, s!(4, EOS), "1.0000 EOS"
        to_string_signed, -1_0000, s!(4, EOS), "-1.0000 EOS"
        to_string_fraction, 1_0001, s!(4, EOS), "1.0001 EOS"
        to_string_zero_precision, 10_001, s!(0, EOS), "10001 EOS"
        to_string_zero_precision_unsigned, -10_001, s!(0, EOS), "-10001 EOS"
    }
}
