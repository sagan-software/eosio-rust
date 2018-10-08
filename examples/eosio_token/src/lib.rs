#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::prelude::*;

#[eosio_action]
fn create(issuer: AccountName, max_supply: Asset) {
    let receiver = current_receiver();
    require_auth(receiver);

    let symbol = max_supply.symbol;
    eosio_assert!(max_supply.amount > 0, "max-supply must be positive");

    let symbol_name = symbol.name();
    let table = CurrencyStats::table(receiver, symbol_name, n!(stat));

    eosio_assert!(
        !table.exists(symbol_name),
        "token with symbol already exists"
    );

    let stats = CurrencyStats {
        supply: Asset { amount: 0, symbol },
        max_supply,
        issuer,
    };

    table.emplace(receiver, stats);
}

#[eosio_action]
fn issue(to: AccountName, quantity: Asset, memo: String) {
    let receiver = current_receiver();
    let symbol = quantity.symbol;

    eosio_assert!(memo.len() <= 256, "memo has more than 256 bytes");
    let table = CurrencyStats::table(receiver, symbol.name(), n!(stat));
    let itr = table
        .find(symbol.name())
        .assert("token with symbol does not exist, create token before issue");

    let mut st = itr.get().assert("read");

    require_auth(st.issuer);
    eosio_assert!(quantity.amount > 0, "must issue positive quantity");
    eosio_assert!(
        quantity.symbol == st.supply.symbol,
        "symbol precision mismatch"
    );
    eosio_assert!(
        quantity.amount <= st.max_supply.amount - st.supply.amount,
        "quantity exceeds available supply"
    );

    st.supply.amount += quantity.amount;
    itr.modify(0, st).assert("write");

    add_balance(st.issuer, quantity, st.issuer);

    if to != st.issuer {
        let args = TransferArgs {
            from: st.issuer,
            to,
            quantity,
            memo,
        };
        args.action(&[PermissionLevel {
            actor: st.issuer,
            permission: n!(active).into(),
        }])
        .send()
        .assert("failed to send inline action");
    }
}

#[eosio_action]
fn open(owner: AccountName, symbol: Symbol, ram_payer: AccountName) {
    require_auth(ram_payer);
    let receiver = current_receiver();
    let accounts_table = Account::table(receiver, symbol.name(), n!(accounts));
    let itr = accounts_table.find(symbol.name());
    if (itr.is_none()) {
        let mut account = Account {
            balance: Asset { amount: 0, symbol },
        };
        accounts_table.emplace(ram_payer, account);
    }
}

#[eosio_action]
fn close(owner: AccountName, symbol: Symbol) {
    require_auth(owner);
    let receiver = current_receiver();
    let accounts_table = Account::table(receiver, symbol.name(), n!(accounts));
    let itr = accounts_table
        .find(symbol.name())
        .assert("Balance row already deleted or never existed. Action won't have any effect.");

    let account = itr.get().assert("read");

    eosio_assert!(
        account.balance.amount == 0,
        "Cannot close because the balance is not zero."
    );
    itr.erase().assert("read");
}

#[eosio_action]
fn retire(quantity: Asset, memo: String) {
    eosio_assert!(memo.len() <= 256, "memo has more than 256 bytes");

    let receiver = current_receiver();
    let symbol = quantity.symbol;
    let stats_table = CurrencyStats::table(receiver, symbol.name(), n!(stat));
    let itr = stats_table
        .find(symbol.name())
        .assert("token with symbol does not exist");

    let mut st = itr.get().assert("error reading stats table");
    require_auth(st.issuer);
    eosio_assert!(quantity.amount > 0, "must retire positive quantity");
    eosio_assert!(
        quantity.symbol == st.supply.symbol,
        "symbol precision mismatch"
    );

    st.supply.amount -= quantity.amount;
    itr.modify(0, st).assert("write");
}

#[eosio_action]
fn transfer(from: AccountName, to: AccountName, quantity: Asset, memo: String) {
    eosio_assert!(from != to, "cannot transfer to self");
    require_auth(from);
    eosio_assert!(is_account(to), "to account does not exist");

    let receiver = current_receiver();
    let symbol_name = quantity.symbol.name();
    let stats_table = CurrencyStats::table(receiver, symbol_name, n!(stat));
    let itr = stats_table
        .find(symbol_name)
        .assert("token with symbol does not exist");
    let st = itr.get().assert("read");

    require_recipient(from);
    require_recipient(to);

    eosio_assert!(quantity.amount > 0, "must transfer positive quantity");
    eosio_assert!(
        quantity.symbol == st.supply.symbol,
        "symbol precision mismatch"
    );
    eosio_assert!(memo.len() <= 256, "memo has more than 256 bytes");

    let payer = if has_auth(to) { to } else { from };

    sub_balance(from, quantity);
    add_balance(to, quantity, payer);
}

eosio_abi!(create, issue, transfer, open, close, retire);

fn sub_balance(owner: AccountName, value: Asset) {
    let receiver = current_receiver();
    let accounts_table = Account::table(receiver, owner, n!(accounts));
    let itr = accounts_table
        .find(value.symbol.name())
        .assert("no balance object found");

    let mut account = itr.get().assert("read");

    account.balance.amount -= value.amount;

    itr.modify(owner, account).assert("write");
}

fn add_balance(owner: AccountName, value: Asset, ram_payer: AccountName) {
    let receiver = current_receiver();
    let accounts_table = Account::table(receiver, owner, n!(accounts));
    let itr = accounts_table.find(value.symbol.name());
    match itr {
        Some(itr) => {
            let mut account = itr.get().assert("read");
            account.balance.amount += value.amount;
            itr.modify(ram_payer, account).assert("write");
        }
        None => {
            let account = Account { balance: value };
            accounts_table.emplace(ram_payer, account).assert("write");
        }
    }
}

#[derive(Read, Write, Copy, Clone)]
struct Account {
    balance: Asset,
}

impl TableRow for Account {
    fn primary_key(&self) -> u64 {
        self.balance.symbol.name().into()
    }
}

#[derive(Read, Write, Copy, Clone)]
struct CurrencyStats {
    supply: Asset,
    max_supply: Asset,
    issuer: AccountName,
}

impl TableRow for CurrencyStats {
    fn primary_key(&self) -> u64 {
        self.supply.symbol.name().into()
    }
}
