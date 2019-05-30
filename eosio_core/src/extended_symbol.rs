use crate::{AccountName, ScopeName, Symbol};
use eosio_bytes::{NumBytes, Read, Write};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy, Default, Read, Write, NumBytes)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct ExtendedSymbol {
    pub symbol: Symbol,
    pub contract: AccountName,
}

impl fmt::Display for ExtendedSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.symbol, self.contract)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eosio_numstr_macros::{n, s};

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
