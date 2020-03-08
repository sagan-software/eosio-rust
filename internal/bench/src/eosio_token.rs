use crate::scenario::{Action, Scenario, Wasm};
use lazy_static::lazy_static;
use rand::{distributions::Distribution, rngs::ThreadRng};
use rand_regex::Regex as RandRegex;
use serde_json::json;
use std::collections::HashSet;

pub struct EosioTokenScenario {
    rng: ThreadRng,
    used_symbols: HashSet<String>,
}

lazy_static! {
    static ref SYMBOL_RE: RandRegex = RandRegex::compile(r"[A-Z]+", 5).unwrap();
}

impl Scenario for EosioTokenScenario {
    const ABI: &'static str =
        "/eosio.contracts/build/contracts/eosio.token/eosio.token.abi";
    const DESC: &'static str = "eosio.token";

    fn new() -> Self {
        let rng = rand::thread_rng();
        Self {
            rng,
            used_symbols: Default::default(),
        }
    }

    fn wasm(&self) -> Vec<Wasm> {
        vec![
            Wasm {
                desc: "eosio.contracts v1.9.0".into(),
                lang: "cpp".into(),
                path: "/eosio.contracts/build/contracts/eosio.token/eosio.\
                       token.wasm"
                    .into(),
            },
            Wasm {
                desc: "eosio-rust v0.3.1".into(),
                lang: "rust".into(),
                path: "/mnt/dev/release/eosio_token_gc.wasm".into(),
            },
        ]
    }

    fn actions(&mut self, contract: &str) -> Vec<Action> {
        let symbol = self.unique_symbol();
        let create = Action::new(
            "create",
            json!(["alice", format!("1000.0000 {}", symbol), symbol]),
            format!("{}@active", contract),
        );
        let issue = Action::new(
            "issue",
            json!(["alice", format!("1.0000 {}", symbol), "here you go"]),
            "alice@active",
        );
        let transfer1 = Action::new(
            "transfer",
            json!([
                "alice",
                "bob",
                format!("1.0000 {}", symbol),
                "here you go"
            ]),
            "alice@active",
        );
        let transfer2 = Action::new(
            "transfer",
            json!([
                "bob",
                "alice",
                format!("1.0000 {}", symbol),
                "here you go"
            ]),
            "bob@active",
        );
        let retire = Action::new(
            "retire",
            json!([format!("1.0000 {}", symbol), "retire"]),
            "alice@active",
        );
        let close = Action::new(
            "close",
            json!(["alice", format!("4,{}", symbol)]),
            "alice@active",
        );
        let open = Action::new(
            "open",
            json!(["alice", format!("4,{}", symbol), "alice"]),
            "alice@active",
        );
        self.used_symbols.insert(symbol);
        vec![create, issue, transfer1, transfer2, retire, close, open]
    }
}

impl EosioTokenScenario {
    fn unique_symbol(&mut self) -> String {
        loop {
            let symbol: String = SYMBOL_RE.sample(&mut self.rng);
            if !self.used_symbols.contains(&symbol) {
                return symbol;
            }
        }
    }
}
