use crate::account::AccountName;
use crate::check::*;
use crate::lib::*;
use crate::symbol::Symbol;
use eosio_macros::*;

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write, NumBytes)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
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

impl ToString for Asset {
    #[inline]
    fn to_string(&self) -> String {
        let precision = self.symbol.precision();
        let amount = (self.amount as f64) / 10_f64.powf(precision as f64);
        let symbol_name = self.symbol.name().to_string();
        let mut s = amount.to_string();
        let mut decimals = if s.contains('.') {
            s.as_str()
                .rsplit('.')
                .next()
                .map_or_else(|| 0_u64, |x| x.len() as u64)
        } else {
            s.push_str(".");
            0_u64
        };
        while decimals < precision {
            s.push_str("0");
            decimals += 1;
        }
        s.push_str(" ");
        s.push_str(&symbol_name);
        s
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

impl Add for Asset {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        check(
            self.symbol == other.symbol,
            "attempt to add asset with different symbol",
        );
        let amount = self
            .amount
            .checked_add(other.amount)
            .check("addition overflow");
        Self {
            amount,
            symbol: self.symbol,
        }
    }
}

impl AddAssign for Asset {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        check(
            self.symbol == other.symbol,
            "attempt to add asset with different symbol",
        );
        let amount = self
            .amount
            .checked_add(other.amount)
            .check("addition overflow");
        self.amount = amount;
    }
}

impl Sub for Asset {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        check(
            self.symbol == other.symbol,
            "attempt to subtract asset with different symbol",
        );
        let amount = self
            .amount
            .checked_sub(other.amount)
            .check("subtraction overflow");
        Self {
            amount,
            symbol: self.symbol,
        }
    }
}

impl SubAssign for Asset {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        check(
            self.symbol == other.symbol,
            "attempt to subtract asset with different symbol",
        );
        let amount = self
            .amount
            .checked_sub(other.amount)
            .check("subtraction overflow");
        self.amount = amount;
    }
}

impl Mul for Asset {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        check(
            self.symbol == other.symbol,
            "attempt to multiply asset with different symbol",
        );
        let amount = self
            .amount
            .checked_mul(other.amount)
            .check("multiplication overflow");
        Self {
            amount,
            symbol: self.symbol,
        }
    }
}

impl MulAssign for Asset {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        check(
            self.symbol == other.symbol,
            "attempt to multiply asset with different symbol",
        );
        let amount = self
            .amount
            .checked_mul(other.amount)
            .check("multiplication overflow");
        self.amount = amount;
    }
}

impl Div for Asset {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        check(
            self.symbol == other.symbol,
            "attempt to divide asset with different symbol",
        );
        check(other.amount != 0, "divide by zero");
        let amount = self
            .amount
            .checked_div(other.amount)
            .check("division overflow");
        Self {
            amount,
            symbol: self.symbol,
        }
    }
}

impl DivAssign for Asset {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        check(
            self.symbol == other.symbol,
            "attempt to divide asset with different symbol",
        );
        check(other.amount != 0, "divide by zero");
        let amount = self
            .amount
            .checked_div(other.amount)
            .check("division overflow");
        self.amount = amount;
    }
}

impl Rem for Asset {
    type Output = Self;
    #[inline]
    fn rem(self, other: Self) -> Self {
        check(
            self.symbol == other.symbol,
            "attempt to remainder asset with different symbol",
        );
        check(other.amount != 0, "remainder by zero");
        let amount = self
            .amount
            .checked_rem(other.amount)
            .check("remainder overflow");
        Self {
            amount,
            symbol: self.symbol,
        }
    }
}

impl RemAssign for Asset {
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        check(
            self.symbol == other.symbol,
            "attempt to remainder asset with different symbol",
        );
        check(other.amount != 0, "remainder by zero");
        let amount = self
            .amount
            .checked_rem(other.amount)
            .check("remainder overflow");
        self.amount = amount;
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write)]
pub struct ExtendedAsset {
    pub quantity: Asset,
    pub contract: AccountName,
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
