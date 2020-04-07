use crate::{
    scenario::{Action, Scenario, Wasm},
    NAME_RE,
};
use rand::{distributions::Distribution, rngs::ThreadRng};
use serde_json::json;

pub struct BenchContracts {
    rng: ThreadRng,
}

impl Scenario for BenchContracts {
    const ABI: &'static str =
        "/mnt/dev/project/target/wasm32-unknown-unknown/release/bench.abi.json";
    const DESC: &'static str = "bench";

    fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    fn wasm(&self) -> Vec<Wasm> {
        vec![Wasm {
            desc: "cpp".into(),
            lang: "cpp".into(),
            path: "/mnt/dev/project/target/wasm32-unknown-unknown/release/bench_cpp.wasm".into(),
        }, Wasm {
            desc: "rust".into(),
            lang: "rust".into(),
            path: "/mnt/dev/project/target/wasm32-unknown-unknown/release/bench_rs_gc.wasm".into()
        }]
    }

    fn actions(&mut self, _contract: &str) -> Vec<Action> {
        let name: String = NAME_RE.sample(&mut self.rng);
        let noop = Action::new("noop", json!([name]), "alice@active");
        vec![noop]
    }
}
