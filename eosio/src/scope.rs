use account::AccountName;
use eosio_macros::*;
use symbol::SymbolName;

eosio_name!(ScopeName);

impl From<AccountName> for ScopeName {
    fn from(account: AccountName) -> Self {
        let value: u64 = account.into();
        value.into()
    }
}

impl From<ScopeName> for AccountName {
    fn from(scope: ScopeName) -> Self {
        let value: u64 = scope.into();
        value.into()
    }
}

impl From<SymbolName> for ScopeName {
    fn from(symbol: SymbolName) -> Self {
        let value: u64 = symbol.into();
        value.into()
    }
}

impl From<ScopeName> for SymbolName {
    fn from(scope: ScopeName) -> Self {
        let value: u64 = scope.into();
        value.into()
    }
}
