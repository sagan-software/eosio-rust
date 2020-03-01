use super::Asset;
use crate::{
    account::AccountName,
    bytes::{NumBytes, Read, Write},
};
use core::{fmt, ops::Deref};

/// Extended asset which stores the information of the owner of the asset
/// <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/asset.hpp#L371-L481>
#[derive(
    Debug, PartialEq, PartialOrd, Clone, Copy, Default, NumBytes, Read, Write,
)]
#[eosio(crate_path = "crate::bytes")]
pub struct ExtendedAsset {
    /// The asset
    pub quantity: Asset,
    /// The owner of the asset
    pub contract: AccountName,
}

impl fmt::Display for ExtendedAsset {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @ {}", self.quantity, self.contract.deref())
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
mod extended_asset_tests {
    use super::{Asset, ExtendedAsset};
    use alloc::string::ToString;
    use eosio_macros::{n, s};

    macro_rules! test_to_string {
        ($($name:ident, $amount:expr, $symbol:expr, $contract:expr, $expected:expr)*) => ($(
            #[test]
            fn $name() {
                let asset = ExtendedAsset {
                    quantity: Asset {
                        amount: $amount,
                        symbol: $symbol.into(),
                    },
                    contract: $contract.into(),
                };
                assert_eq!(asset.to_string(), $expected);
            }
        )*)
    }

    test_to_string! {
        to_string,
        1_0000,
        s!(4, "EOS"),
        n!("eosio.token"),
        "1.0000 EOS @ eosio.token"

        to_string_signed,
        -1_0000,
        s!(4, "EOS"),
        n!("eosio.token"),
        "-1.0000 EOS @ eosio.token"

        to_string_decimal,
        1_0001,
        s!(4, "EOS"),
        n!("eosio.token"),
        "1.0001 EOS @ eosio.token"

        to_string_zero_precision,
        10_001,
        s!(0, "EOS"),
        n!("eosio.token"),
        "10001 EOS @ eosio.token"

        to_string_zero_precision_signed,
        -10_001,
        s!(0, "EOS"),
        n!("eosio.token"),
        "-10001 EOS @ eosio.token"
    }
}
