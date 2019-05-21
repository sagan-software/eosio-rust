use eosio::*;

#[derive(Read, Write, NumBytes, Copy, Clone)]
pub struct Account {
    balance: Asset,
}

#[cfg(feature = "contract")]
impl TableRow for Account {
    const TABLE_NAME: u64 = n!(accounts);

    fn primary_key(&self) -> u64 {
        self.balance.symbol.code().into()
    }
}

#[derive(Read, Write, NumBytes, Copy, Clone)]
pub struct CurrencyStats {
    supply: Asset,
    max_supply: Asset,
    issuer: AccountName,
}

#[cfg(feature = "contract")]
impl TableRow for CurrencyStats {
    const TABLE_NAME: u64 = n!(stat);

    fn primary_key(&self) -> u64 {
        self.supply.symbol.code().into()
    }
}

#[eosio::action]
fn create(issuer: AccountName, max_supply: Asset) {
    let _self = current_receiver();
    require_auth(_self);

    let symbol = max_supply.symbol;
    check(symbol.is_valid(), "invalid symbol name");
    check(max_supply.is_valid(), "invalid supply");
    check(max_supply.amount > 0, "max-supply must be positive");

    let symbol_code = symbol.code();
    let stats_table = CurrencyStats::table(_self, symbol_code);
    check(
        stats_table.find(symbol_code).is_none(),
        "token with symbol already exists",
    );

    let stats = CurrencyStats {
        supply: Asset { amount: 0, symbol },
        max_supply,
        issuer,
    };

    stats_table.emplace(_self, &stats).check("write");
}

#[eosio::action]
fn issue(to: AccountName, quantity: Asset, memo: String) {
    let symbol = quantity.symbol;
    check(symbol.is_valid(), "invalid symbol name");
    check(memo.len() <= 256, "memo has more than 256 bytes");

    let _self = current_receiver();
    let symbol_code = symbol.code();
    let stats_table = CurrencyStats::table(_self, symbol_code);
    let cursor = stats_table
        .find(symbol_code)
        .check("token with symbol does not exist, create token before issue");

    let mut st = cursor.get().check("read");
    require_auth(st.issuer);
    check(quantity.is_valid(), "invalid quantity");
    check(quantity.amount > 0, "must issue positive quantity");
    check(
        quantity.symbol == st.supply.symbol,
        "symbol precision mismatch",
    );
    check(
        quantity.amount <= st.max_supply.amount - st.supply.amount,
        "quantity exceeds available supply",
    );

    st.supply += quantity;
    cursor.modify(None, &st).check("write");

    add_balance(st.issuer, quantity, st.issuer);

    if to != st.issuer {
        let action = TransferAction {
            from: st.issuer,
            to,
            quantity,
            memo,
        };
        action
            .send_inline(vec![Authorization {
                actor: st.issuer,
                permission: n!(active).into(),
            }])
            .check("failed to send inline action");
    }
}

#[eosio::action]
fn retire(quantity: Asset, memo: String) {
    let symbol = quantity.symbol;
    check(symbol.is_valid(), "invalid symbol name");
    check(memo.len() <= 256, "memo has more than 256 bytes");

    let _self = current_receiver();
    let symbol_code = symbol.code();
    let stats_table = CurrencyStats::table(_self, symbol_code);
    let cursor = stats_table
        .find(symbol_code)
        .check("token with symbol does not exist");

    let mut st = cursor.get().check("error reading stats table");
    require_auth(st.issuer);
    check(quantity.is_valid(), "invalid quantity");
    check(quantity.amount > 0, "must retire positive quantity");
    check(symbol == st.supply.symbol, "symbol precision mismatch");

    st.supply -= quantity;
    cursor.modify(None, &st).check("write");
    sub_balance(st.issuer, quantity);
}

#[eosio::action]
fn transfer(from: AccountName, to: AccountName, quantity: Asset, memo: String) {
    check(from != to, "cannot transfer to self");
    require_auth(from);
    check(is_account(to), "to account does not exist");

    let _self = current_receiver();
    let symbol_code = quantity.symbol.code();
    let stats_table = CurrencyStats::table(_self, symbol_code);
    let cursor = stats_table
        .find(symbol_code)
        .check("token with symbol does not exist");
    let st = cursor.get().check("read");

    require_recipient(from);
    require_recipient(to);

    check(quantity.is_valid(), "invalid quantity");
    check(quantity.amount > 0, "must transfer positive quantity");
    check(
        quantity.symbol == st.supply.symbol,
        "symbol precision mismatch",
    );
    check(memo.len() <= 256, "memo has more than 256 bytes");

    let payer = if has_auth(to) { to } else { from };

    sub_balance(from, quantity);
    add_balance(to, quantity, payer);
}

#[cfg(feature = "contract")]
fn sub_balance(owner: AccountName, value: Asset) {
    let _self = current_receiver();
    let table = Account::table(_self, owner);
    let cursor = table
        .find(value.symbol.code())
        .check("no balance object found");
    let mut from = cursor.get().check("read");
    check(from.balance.amount >= value.amount, "overdrawn balance");

    from.balance -= value;
    cursor.modify(None, &from).check("write");
}

#[cfg(feature = "contract")]
fn add_balance(owner: AccountName, value: Asset, ram_payer: AccountName) {
    let _self = current_receiver();
    let accounts_table = Account::table(_self, owner);
    let cursor = accounts_table.find(value.symbol.code());
    match cursor {
        Some(cursor) => {
            let mut account = cursor.get().check("read");
            account.balance += value;
            cursor.modify(Some(ram_payer), &account).check("write");
        }
        None => {
            let account = Account { balance: value };
            accounts_table.emplace(ram_payer, &account).check("write");
        }
    }
}

#[eosio::action]
fn open(owner: AccountName, symbol: Symbol, ram_payer: AccountName) {
    require_auth(ram_payer);
    let _self = current_receiver();
    let symbol_code = symbol.code();

    let stats_table = CurrencyStats::table(_self, symbol_code);
    let st = stats_table
        .find(symbol_code)
        .check("symbol does not exist")
        .get()
        .check("read");
    check(st.supply.symbol == symbol, "symbol precision mismatch");

    let accts_table = Account::table(_self, owner);
    if accts_table.find(symbol_code).is_none() {
        let account = Account {
            balance: Asset { amount: 0, symbol },
        };
        accts_table.emplace(ram_payer, &account).check("write");
    }
}

#[eosio::action]
fn close(owner: AccountName, symbol: Symbol) {
    require_auth(owner);
    let _self = current_receiver();
    let accts_table = Account::table(_self, owner);
    let accts_cursor = accts_table
        .find(symbol.code())
        .check("Balance row already deleted or never existed. Action won't have any effect.");
    let account = accts_cursor.get().check("read");
    check(
        account.balance.amount == 0,
        "Cannot close because the balance is not zero.",
    );
    accts_cursor.erase().check("read");
}

eosio::abi!(create, issue, transfer, open, close, retire);
