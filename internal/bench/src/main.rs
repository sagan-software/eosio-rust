mod bench_contracts;
mod eosio_token;
mod scenario;

use crate::{
    bench_contracts::BenchContracts,
    eosio_token::EosioTokenScenario,
    scenario::{Scenario, Wasm},
};
use lazy_static::lazy_static;
use rand::distributions::Distribution;
use rand_regex::Regex as RandRegex;
use serde::Serialize;
use std::{collections::BTreeMap, fs::File};

const PUBKEY: &str = "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV";
const TOTAL_ITERATIONS_PER_WASM: usize = 500;
const WARMUP_ITERATIONS: usize = 20;

lazy_static! {
    pub static ref SYMBOL_RE: RandRegex =
        RandRegex::compile(r"[A-Z]{5}", 5).unwrap();
    pub static ref NAME_RE: RandRegex =
        RandRegex::compile(r"[a-z12345]{12}", 12).unwrap();
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let file = File::create("/mnt/dev/perf-data/data.json")
        .expect("Failed to create output file");
    let scenario = BenchContracts::new();
    // let output =
    // let scenario = EosioTokenScenario::new();
    let output = run_scenario(scenario);
    serde_json::to_writer(file, &output).expect("Failed to write to file");
    log::info!("Done");
}

fn run_scenario<T: Scenario>(mut scenario: T) -> Output {
    log::info!("Running benchmarks for scenario: {}", T::DESC);

    let mut results = Vec::new();
    for wasm in scenario.wasm() {
        let output = run_scenario_wasm(&mut scenario, wasm);
        log::info!(
            "Done with WASM {}. Summary:\n{}",
            &output.name,
            output
                .actions
                .iter()
                .map(|(k, v)| format!(
                    "    {}: avg={}, min={}, max={}",
                    k, v.min, v.avg, v.max
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );
        results.push(output);
    }

    log::info!("Done with scenario {}", T::DESC);

    Output {
        scenario: T::DESC.to_string(),
        results,
    }
}

fn run_scenario_wasm<T: Scenario>(scenario: &mut T, wasm: Wasm) -> WasmOutput {
    log::info!("Running benchmarks for WASM: {}", &wasm.desc);
    let account = random_account_name();
    util::new_account(
        &account,
        PUBKEY,
        "1000.0000 EOS",
        "1000.0000 EOS",
        "1000",
    );
    log::info!(
        "Setting contract: account = '{}', abi = '{}', wasm = '{}'",
        account,
        T::ABI,
        wasm.path
    );
    util::set_abi(&account, T::ABI);
    let code_metrics = util::set_code_metrics(&account, &wasm.path);
    let mut runs = Vec::new();
    log::info!("Warming up for {} iterations...", WARMUP_ITERATIONS);
    for i in 0..TOTAL_ITERATIONS_PER_WASM {
        if i % 50 == 0 {
            log::info!("Iteration #{}", i);
        } else {
            log::debug!("Iteration #{}", i);
        }
        log::debug!("Iteration #{}", i);
        if i == WARMUP_ITERATIONS {
            log::info!("Done warming up! Saving results...");
        }

        for action in scenario.actions(&account) {
            let run = action.run(&account);
            if i >= WARMUP_ITERATIONS {
                runs.push(run);
            }
        }
    }
    let runs = runs.into_iter().fold(BTreeMap::new(), |mut acc, r| {
        let times = acc.entry(r.action).or_insert_with(Vec::new);
        times.push(r.time);
        acc
    });
    let actions = runs
        .into_iter()
        .map(|(key, mut runs)| {
            runs.sort();
            let sum: u64 = runs.iter().sum();
            let avg = sum / (runs.len() as u64);
            let min = runs.iter().min().unwrap();
            let max = runs.iter().max().unwrap();
            (
                key,
                WasmActionSummary {
                    avg,
                    min: *min,
                    max: *max,
                    runs,
                },
            )
        })
        .collect();
    WasmOutput {
        name: wasm.desc,
        ram: code_metrics.bytes,
        actions,
    }
}

fn random_account_name() -> String {
    let mut rng = rand::thread_rng();
    NAME_RE.sample(&mut rng)
}

#[derive(Serialize)]
struct Output {
    scenario: String,
    results: Vec<WasmOutput>,
}

#[derive(Serialize)]
struct WasmOutput {
    name: String,
    ram: u64,
    actions: BTreeMap<String, WasmActionSummary>,
}

#[derive(Serialize)]
struct WasmActionSummary {
    avg: u64,
    min: u64,
    max: u64,
    runs: Vec<u64>,
}
