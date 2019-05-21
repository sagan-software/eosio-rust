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

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct ExtendedAsset {
    pub quantity: Asset,
    pub contract: AccountName,
}

impl fmt::Display for ExtendedAsset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @ {}", self.quantity, self.contract)
    }
}

// impl Add for ExtendedAsset {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         check(self.contract == other.contract, "type mismatch");
//         ExtendedAsset {
//             quantity: self.quantity + other.quantity,
//             contract: self.contract,
//         }
//     }
// }

// impl AddAssign for ExtendedAsset {
//     fn add_assign(&mut self, other: Self) {
//         check(self.contract == other.contract, "type mismatch");
//         self.quantity += other.quantity
//     }
// }

// impl Sub for ExtendedAsset {
//     type Output = Self;
//     fn sub(self, other: Self) -> Self {
//         check(self.contract == other.contract, "type mismatch");
//         ExtendedAsset {
//             quantity: self.quantity - other.quantity,
//             contract: self.contract,
//         }
//     }
// }

// impl SubAssign for ExtendedAsset {
//     fn sub_assign(&mut self, other: Self) {
//         check(self.contract == other.contract, "type mismatch");
//         self.quantity -= other.quantity
//     }
// }

// impl Mul for ExtendedAsset {
//     type Output = Self;
//     fn mul(self, other: Self) -> Self {
//         check(self.contract == other.contract, "type mismatch");
//         ExtendedAsset {
//             quantity: self.quantity * other.quantity,
//             contract: self.contract,
//         }
//     }
// }

// impl MulAssign for ExtendedAsset {
//     fn mul_assign(&mut self, other: Self) {
//         check(self.contract == other.contract, "type mismatch");
//         self.quantity *= other.quantity
//     }
// }

// impl Div for ExtendedAsset {
//     type Output = Self;
//     fn div(self, other: Self) -> Self {
//         check(self.contract == other.contract, "type mismatch");
//         ExtendedAsset {
//             quantity: self.quantity / other.quantity,
//             contract: self.contract,
//         }
//     }
// }

// impl DivAssign for ExtendedAsset {
//     fn div_assign(&mut self, other: Self) {
//         check(self.contract == other.contract, "type mismatch");
//         self.quantity /= other.quantity
//     }
// }

// impl Rem for ExtendedAsset {
//     type Output = Self;
//     fn rem(self, other: Self) -> Self {
//         check(self.contract == other.contract, "type mismatch");
//         ExtendedAsset {
//             quantity: self.quantity % other.quantity,
//             contract: self.contract,
//         }
//     }
// }

// impl RemAssign for ExtendedAsset {
//     fn rem_assign(&mut self, other: Self) {
//         check(self.contract == other.contract, "type mismatch");
//         self.quantity %= other.quantity
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use eosio_numstr_macros::{n, s};

    #[test]
    fn to_string() {
        fn test(amount: i64, symbol: u64, expected: &str) {
            let asset = Asset {
                amount,
                symbol: symbol.into(),
            };
            assert_eq!(asset.to_string(), expected);
        }

        test(1_0000, s!(4, EOS), "1.0000 EOS");
        test(-1_0000, s!(4, EOS), "-1.0000 EOS");
        test(1_0001, s!(4, EOS), "1.0001 EOS");
        test(10_001, s!(0, EOS), "10001 EOS");
        test(-10_001, s!(0, EOS), "-10001 EOS");
    }

    #[test]
    fn extended_to_string() {
        fn test(amount: i64, symbol: u64, contract: u64, expected: &str) {
            let asset = ExtendedAsset {
                quantity: Asset {
                    amount,
                    symbol: symbol.into(),
                },
                contract: contract.into(),
            };
            assert_eq!(asset.to_string(), expected);
        }
        test(
            1_0000,
            s!(4, EOS),
            n!(eosio.token),
            "1.0000 EOS @ eosio.token",
        );
        test(
            -1_0000,
            s!(4, EOS),
            n!(eosio.token),
            "-1.0000 EOS @ eosio.token",
        );
        test(
            1_0001,
            s!(4, EOS),
            n!(eosio.token),
            "1.0001 EOS @ eosio.token",
        );
        test(
            10_001,
            s!(0, EOS),
            n!(eosio.token),
            "10001 EOS @ eosio.token",
        );
        test(
            -10_001,
            s!(0, EOS),
            n!(eosio.token),
            "-10001 EOS @ eosio.token",
        );
    }
}
