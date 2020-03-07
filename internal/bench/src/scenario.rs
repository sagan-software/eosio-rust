use serde_json::Value;

pub trait Scenario {
    const DESC: &'static str;
    const ABI: &'static str;
    fn new() -> Self;
    fn wasm(&self) -> Vec<Wasm>;
    fn actions(&mut self, contract: &str) -> Vec<Action>;
}

#[derive(Debug)]
pub struct Wasm {
    pub desc: String,
    pub lang: String,
    pub path: String,
}

#[derive(Debug)]
pub struct Action {
    pub name: String,
    pub data: Value,
    pub scope: String,
}

#[derive(Debug)]
pub struct Run {
    pub action: String,
    pub bytes: u64,
    pub time: u64,
}

pub struct WasmBenchResults {
    pub wasm: Wasm,
    pub ram: u64,
    pub runs: Vec<Run>,
}

impl Action {
    pub fn new(
        name: impl ToString,
        data: impl Into<Value>,
        scope: impl ToString,
    ) -> Self {
        Self {
            name: name.to_string(),
            data: data.into(),
            scope: scope.to_string(),
        }
    }

    pub fn run(&self, contract: impl AsRef<str>) -> Run {
        let contract = contract.as_ref();
        let data = serde_json::to_string(&self.data).unwrap();
        println!(
            "push action {} {} {} -p {}",
            contract, self.name, data, self.scope
        );
        let metrics =
            util::push_action_metrics(contract, &self.name, data, &self.scope);
        Run {
            action: self.name.clone(),
            bytes: metrics.bytes,
            time: metrics.time,
        }
    }
}
