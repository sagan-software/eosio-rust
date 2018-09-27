use account::AccountName;
use eosio_macros::*;
use symbol::SymbolName;

eosio_name!(ScopeName);

impl From<AccountName> for ScopeName {
    fn from(account: AccountName) -> ScopeName {
        let value: u64 = account.into();
        value.into()
    }
}

impl From<SymbolName> for ScopeName {
    fn from(symbol: SymbolName) -> ScopeName {
        let value: u64 = symbol.into();
        value.into()
    }
}
