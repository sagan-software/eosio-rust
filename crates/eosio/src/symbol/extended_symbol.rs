use super::Symbol;
use crate::{
    account::AccountName,
    bytes::{NumBytes, Read, Write},
};
use core::{fmt, ops::Deref};
pub use eosio_numstr::ParseSymbolError;

/// Extended asset which stores the information of the owner of the symbol
/// <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/symbol.hpp#L372-L450>
#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write, NumBytes)]
#[eosio(crate_path = "crate::bytes")]
pub struct ExtendedSymbol {
    /// The symbol
    pub symbol: Symbol,
    /// The token contract hosting the symbol
    pub contract: AccountName,
}

impl fmt::Display for ExtendedSymbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.symbol, self.contract.deref())
    }
}

#[cfg(test)]
mod tests {
    use super::{AccountName, ExtendedSymbol, Symbol};
    use crate::SymbolCode;
    use alloc::string::ToString;
    use core::str::FromStr;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn from_str_to_string(
            precision in 0_u8..,
            code in "[A-Z]{2,7}",
            contract in "[[1-5][a-z]]{1,12}"
        ) {
            let expected = format!("{},{}@{}", precision, code, contract);
            let code = SymbolCode::from_str(&code).unwrap();
            let symbol = Symbol::new_with_code(precision, code);
            let contract = AccountName::from_str(&contract).unwrap();
            let extended_symbol = ExtendedSymbol { symbol, contract };
            let result = extended_symbol.to_string();
            prop_assert_eq!(result, expected);
        }
    }
}

// #[cfg(test)]
// mod extended_symbol_tests {
//     use super::*;
//     use alloc::string::ToString;
//     use eosio_macros::{n, s};

//     macro_rules! test_to_string {
//         ($($name:ident, $symbol:expr, $contract:expr, $expected:expr)*) =>
// ($(             #[test]
//             fn $name() {
//                 let extended = ExtendedSymbol {
//                     symbol: $symbol.into(),
//                     contract: $contract.into(),
//                 };
//                 assert_eq!(extended.to_string(), $expected);
//             }
//         )*)
//     }

//     test_to_string! {
//         to_string,
//         s!(4, "EOS"),
//         n!("eosio.token"),
//         "4,EOS@eosio.token"

//         to_string_zero_precision,
//         s!(0, "TST"),
//         n!("test"),
//         "0,TST@test"

//         to_string_one_precision,
//         s!(1, "TGFT"),
//         n!("greatfiltert"),
//         "1,TGFT@greatfiltert"
//     }
// }
