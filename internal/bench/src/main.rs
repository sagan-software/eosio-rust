mod eosio_token;
mod scenario;

use crate::{
    eosio_token::EosioTokenScenario,
    scenario::{Scenario, WasmBenchResults},
};
use rand::Rng;
use serde::Serialize;
use std::{collections::HashMap, fs::File};

const PUBKEY: &str = "EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV";
const TOTAL_ITERATIONS_PER_WASM: usize = 500;
const WARMUP_ITERATIONS: usize = 20;

fn main() {
    let scenario = EosioTokenScenario::new();
    run_scenario(scenario);
}

fn run_scenario<T: Scenario>(mut scenario: T) {
    println!("Running benchmarks for scenario: {}", T::DESC);

    let mut results = Vec::new();
    for wasm in scenario.wasm() {
        println!("Running benchmarks for WASM: {}", &wasm.desc);
        let account = random_account_name();
        util::new_account(
            &account,
            PUBKEY,
            "1000.0000 EOS",
            "1000.0000 EOS",
            "1000",
        );
        println!(
            "Setting contract: account = '{}', abi = '{}', wasm = '{}'",
            account,
            T::ABI,
            wasm.path
        );
        util::set_abi(&account, T::ABI);
        let code_metrics = util::set_code_metrics(&account, &wasm.path);
        let mut runs = Vec::new();
        println!("Warming up for {} iterations...", WARMUP_ITERATIONS);
        for i in 0..TOTAL_ITERATIONS_PER_WASM {
            println!("Iteration #{}", i);
            if i == WARMUP_ITERATIONS {
                println!("Done warming up! Saving results...");
            }

            for action in scenario.actions(&account) {
                let run = action.run(&account);
                if i >= WARMUP_ITERATIONS {
                    runs.push(run);
                }
            }
        }

        results.push(WasmBenchResults {
            wasm,
            ram: code_metrics.bytes,
            runs,
        });
    }

    let output = Output {
        scenario: T::DESC.to_string(),
        results: results
            .into_iter()
            .map(|r| {
                let runs =
                    r.runs.into_iter().fold(HashMap::new(), |mut acc, r| {
                        let times =
                            acc.entry(r.action).or_insert_with(Vec::new);
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
                        (key, WasmActionSummary {
                            avg,
                            min: *min,
                            max: *max,
                            runs,
                        })
                    })
                    .collect();
                WasmOutput {
                    name: r.wasm.desc,
                    ram: r.ram,
                    actions,
                }
            })
            .collect(),
    };

    let file = File::create("/tmp5.json").unwrap();

    serde_json::to_writer(file, &output).unwrap();

    println!("Done");
}

fn random_account_name() -> String {
    let mut rng = rand::thread_rng();
    let gen = rand_regex::Regex::compile(r"[a-z12345]+", 12).unwrap();
    (&mut rng).sample(&gen)
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
    actions: HashMap<String, WasmActionSummary>,
}

#[derive(Serialize)]
struct WasmActionSummary {
    avg: u64,
    min: u64,
    max: u64,
    runs: Vec<u64>,
}
