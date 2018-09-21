#![no_std]
#![feature(alloc, proc_macro_non_items)]

#[macro_use]
extern crate alloc;
extern crate eosio;
extern crate eosio_bytes;
extern crate eosio_sys;
extern crate eosio_types;

use alloc::prelude::{String, ToString, Vec};
use eosio::prelude::*;

#[eosio_action]
fn create(issuer: AccountName, max_supply: Asset) {
    let receiver = current_receiver();
    require_auth(receiver);

    let symbol = max_supply.symbol;
    eosio_assert!(max_supply.amount > 0, "max-supply must be positive");

    let stats_table = CurrencyStats::table(receiver, symbol.name(), n!(stat));
    eosio_assert!(
        !stats_table.exists(symbol.name()),
        "token with symbol already exists"
    );

    let stats = CurrencyStats {
        supply: Asset { amount: 0, symbol },
        max_supply,
        issuer,
    };

    stats_table.emplace(receiver, stats);
}

#[eosio_action]
fn issue(to: AccountName, quantity: Asset, memo: String) {
    let receiver = current_receiver();
    let symbol = quantity.symbol;

    eosio_assert!(memo.len() <= 256, "memo has more than 256 bytes");
    let stats_table = CurrencyStats::table(receiver, symbol.name(), n!(stat));
    let itr = stats_table.find(symbol.name());
    eosio_assert!(
        !stats_table.is_end(itr),
        "token with symbol does not exist, create token before issue"
    );
    let mut st = stats_table.get(itr).unwrap();

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
    stats_table.modify(itr, 0, st);

    add_balance(st.issuer, quantity, st.issuer);

    if to != st.issuer {
        send_inline(Action {
            account: receiver,
            name: n!(transfer).into(),
            authorization: vec![PermissionLevel {
                actor: st.issuer,
                permission: n!(active).into(),
            }],
            data: TransferArgs {
                from: receiver,
                to,
                quantity: quantity,
                memo,
            },
        });
    }
}

#[eosio_action]
fn open(owner: AccountName, symbol: Symbol, ram_payer: AccountName) {
    require_auth(ram_payer);
    let receiver = current_receiver();
    let accounts_table = Account::table(receiver, symbol.name(), n!(accounts));
    let itr = accounts_table.find(symbol.name());
    if (itr == accounts_table.end()) {
        let mut account = accounts_table.get(itr).unwrap();
        account.balance = Asset { amount: 0, symbol };
        accounts_table.emplace(ram_payer, account);
    }
}

#[eosio_action]
fn close(owner: AccountName, symbol: Symbol) {
    require_auth(owner);
    let receiver = current_receiver();
    let accounts_table = Account::table(receiver, symbol.name(), n!(accounts));
    let itr = accounts_table.find(symbol.name());
    eosio_assert!(
        !accounts_table.is_end(itr),
        "Balance row already deleted or never existed. Action won't have any effect."
    );

    let account = accounts_table.get(itr).unwrap();
    eosio_assert!(
        account.balance.amount == 0,
        "Cannot close because the balance is not zero."
    );
    accounts_table.erase(itr);
}

#[eosio_action]
fn retire(quantity: Asset, memo: String) {
    eosio_assert!(memo.len() <= 256, "memo has more than 256 bytes");

    let receiver = current_receiver();
    let symbol = quantity.symbol;
    let stats_table = CurrencyStats::table(receiver, symbol.name(), n!(stat));
    let itr = stats_table.find(symbol.name());
    eosio_assert!(!stats_table.is_end(itr), "token with symbol does not exist");

    let mut st = stats_table.get(itr).unwrap();
    require_auth(st.issuer);
    eosio_assert!(quantity.amount > 0, "must retire positive quantity");
    eosio_assert!(
        quantity.symbol == st.supply.symbol,
        "symbol precision mismatch"
    );

    st.supply.amount -= quantity.amount;
    stats_table.modify(itr, 0, st);
}

#[eosio_action]
fn transfer(from: AccountName, to: AccountName) {
    let quantity = Asset {
        amount: 1,
        symbol: s!(2, TGFT).into(),
    };
    let memo = "".to_string();
    eosio_assert!(from != to, "cannot transfer to self");
    require_auth(from);
    eosio_assert!(is_account(to), "to account does not exist");
    let receiver = current_receiver();
    let symbol_name = quantity.symbol.name();
    let stats_table = CurrencyStats::table(receiver, symbol_name, n!(stat));
    let itr = stats_table.find(symbol_name);
    eosio_assert!(!stats_table.is_end(itr), "token with symbol does not exist");
    let st = stats_table.get(itr).unwrap();

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
    let itr = accounts_table.find(value.symbol.name());
    eosio_assert!(!accounts_table.is_end(itr), "no balance object found");

    let mut account = accounts_table.get(itr).unwrap();
    account.balance.amount -= value.amount;
    accounts_table.modify(itr, owner, account);
}

fn add_balance(owner: AccountName, value: Asset, ram_payer: AccountName) {
    let receiver = current_receiver();
    let accounts_table = Account::table(receiver, owner, n!(accounts));
    let itr = accounts_table.find(value.symbol.name());
    if accounts_table.is_end(itr) {
        let account = Account { balance: value };
        accounts_table.emplace(ram_payer, account);
    } else {
        let mut account = accounts_table.get(itr).unwrap();
        account.balance.amount += value.amount;
        accounts_table.modify(itr, 0, account);
    }
}

#[derive(Readable, Writeable, Copy, Clone)]
struct Account {
    balance: Asset,
}

impl TableRow for Account {
    fn primary_key(&self) -> u64 {
        self.balance.symbol.name()
    }
}

#[derive(Readable, Writeable, Copy, Clone)]
struct CurrencyStats {
    supply: Asset,
    max_supply: Asset,
    issuer: AccountName,
}

impl TableRow for CurrencyStats {
    fn primary_key(&self) -> u64 {
        self.supply.symbol.name()
    }
}

#[derive(Readable, Writeable, Clone)]
struct TransferArgs {
    from: AccountName,
    to: AccountName,
    quantity: Asset,
    memo: String,
}
