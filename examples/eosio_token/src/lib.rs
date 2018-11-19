#![feature(proc_macro_hygiene)]

use eosio::*;

#[eosio_action]
fn create(issuer: AccountName, max_supply: Asset) {
    let receiver = AccountName::receiver();
    require_auth(receiver);

    let symbol = max_supply.symbol;
    eosio_assert(max_supply.amount > 0, "max-supply must be positive");

    let symbol_name = symbol.name();
    let table = CurrencyStats::table(receiver, symbol_name);

    eosio_assert(
        !table.exists(symbol_name),
        "token with symbol already existss",
    );

    let stats = CurrencyStats {
        supply: Asset { amount: 0, symbol },
        max_supply,
        issuer,
    };

    table.emplace(receiver, &stats).assert("write");
}

#[eosio_action]
fn issue(to: AccountName, quantity: Asset, memo: String) {
    let receiver = AccountName::receiver();
    let symbol = quantity.symbol;

    eosio_assert(memo.len() <= 256, "memo has more than 256 bytes");
    let table = CurrencyStats::table(receiver, symbol.name());
    let cursor = table
        .find(symbol.name())
        .assert("token with symbol does not exist, create token before issue");

    let mut st = cursor.get().assert("read");
    require_auth(st.issuer);

    eosio_assert(quantity.amount > 0, "must issue positive quantity");
    eosio_assert(
        quantity.symbol == st.supply.symbol,
        "symbol precision mismatch",
    );
    eosio_assert(
        quantity.amount <= st.max_supply.amount - st.supply.amount,
        "quantity exceeds available supply",
    );

    st.supply += quantity;
    cursor.modify(None, &st).assert("write");

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
            .assert("failed to send inline action");
    }
}

#[eosio_action]
fn open(owner: AccountName, symbol: Symbol, ram_payer: AccountName) {
    require_auth(ram_payer);
    let receiver = AccountName::receiver();
    let accounts_table = Account::table(receiver, symbol.name());
    let cursor = accounts_table.find(symbol.name());
    if cursor.is_none() {
        let account = Account {
            balance: Asset { amount: 0, symbol },
        };
        accounts_table.emplace(ram_payer, &account).assert("write");
    }
}

#[eosio_action]
fn close(owner: AccountName, symbol: Symbol) {
    require_auth(owner);
    let receiver = AccountName::receiver();
    let accounts_table = Account::table(receiver, symbol.name());
    let cursor = accounts_table
        .find(symbol.name())
        .assert("Balance row already deleted or never existed. Action won't have any effect.");

    let account = cursor.get().assert("read");

    eosio_assert(
        account.balance.amount == 0,
        "Cannot close because the balance is not zero.",
    );
    cursor.erase().assert("read");
}

#[eosio_action]
fn retire(quantity: Asset, memo: String) {
    eosio_assert(memo.len() <= 256, "memo has more than 256 bytes");

    let receiver = AccountName::receiver();
    let symbol = quantity.symbol;
    let stats_table = CurrencyStats::table(receiver, symbol.name());
    let cursor = stats_table
        .find(symbol.name())
        .assert("token with symbol does not exist");

    let mut st = cursor.get().assert("error reading stats table");
    require_auth(st.issuer);
    eosio_assert(quantity.amount > 0, "must retire positive quantity");
    eosio_assert(
        quantity.symbol == st.supply.symbol,
        "symbol precision mismatch",
    );

    st.supply -= quantity;
    cursor.modify(None, &st).assert("write");
}

#[eosio_action]
fn transfer(from: AccountName, to: AccountName, quantity: Asset, memo: String) {
    eosio_assert(from != to, "cannot transfer to self");
    require_auth(from);
    to.is_account().assert("to account does not exist");

    let receiver = AccountName::receiver();
    let symbol_name = quantity.symbol.name();
    let stats_table = CurrencyStats::table(receiver, symbol_name);
    let cursor = stats_table
        .find(symbol_name)
        .assert("token with symbol does not exist");
    let st = cursor.get().assert("read");

    require_recipient(from);
    require_recipient(to);

    eosio_assert(quantity.amount > 0, "must transfer positive quantity");
    eosio_assert(
        quantity.symbol == st.supply.symbol,
        "symbol precision mismatch",
    );
    eosio_assert(memo.len() <= 256, "memo has more than 256 bytes");

    let payer = if to.has_auth() { to } else { from };

    sub_balance(from, quantity);
    add_balance(to, quantity, payer);
}

eosio_abi!(create, issue, transfer, open, close, retire);

fn sub_balance(owner: AccountName, value: Asset) {
    let receiver = AccountName::receiver();
    let table = Account::table(receiver, owner);
    let cursor = table
        .find(value.symbol.name())
        .assert("no balance object found");

    let mut account = cursor.get().assert("read");

    account.balance -= value;

    cursor.modify(Some(owner), &account).assert("write");
}

fn add_balance(owner: AccountName, value: Asset, ram_payer: AccountName) {
    let receiver = AccountName::receiver();
    let accounts_table = Account::table(receiver, owner);
    let cursor = accounts_table.find(value.symbol.name());
    match cursor {
        Some(cursor) => {
            let mut account = cursor.get().assert("read");
            account.balance += value;
            cursor.modify(Some(ram_payer), &account).assert("write");
        }
        None => {
            let account = Account { balance: value };
            accounts_table.emplace(ram_payer, &account).assert("write");
        }
    }
}

#[derive(Read, Write, NumBytes, Copy, Clone)]
struct Account {
    balance: Asset,
}

impl TableRow for Account {
    const NAME: u64 = n!(accounts);

    fn primary_key(&self) -> u64 {
        self.balance.symbol.name().into()
    }
}

#[derive(Read, Write, NumBytes, Copy, Clone)]
struct CurrencyStats {
    supply: Asset,
    max_supply: Asset,
    issuer: AccountName,
}

impl TableRow for CurrencyStats {
    const NAME: u64 = n!(stat);

    fn primary_key(&self) -> u64 {
        self.supply.symbol.name().into()
    }
}
