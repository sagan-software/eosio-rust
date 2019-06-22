//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/symbol.hpp#L372-L450>
use crate::{AccountName, NumBytes, Read, Symbol, Write};
use std::fmt;

/// Extended asset which stores the information of the owner of the symbol
#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write, NumBytes)]
#[eosio_core_root_path = "crate"]
pub struct ExtendedSymbol {
    /// The symbol
    pub symbol: Symbol,
    /// The token contract hosting the symbol
    pub contract: AccountName,
}

impl fmt::Display for ExtendedSymbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.symbol, self.contract)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eosio_core_macros::{n, s};

    macro_rules! test_to_string {
        ($($name:ident, $symbol:expr, $contract:expr, $expected:expr)*) => ($(
            #[test]
            fn $name() {
                let extended = ExtendedSymbol {
                    symbol: $symbol.into(),
                    contract: $contract.into(),
                };
                assert_eq!(extended.to_string(), $expected);
            }
        )*)
    }

    test_to_string! {
        to_string,
        s!(4, EOS),
        n!(eosio.token),
        "4,EOS@eosio.token"

        to_string_zero_precision,
        s!(0, TST),
        n!(test),
        "0,TST@test"

        to_string_one_precision,
        s!(1, TGFT),
        n!(greatfiltert),
        "1,TGFT@greatfiltert"
    }

}
