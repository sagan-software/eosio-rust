use eosio_numstr::{SYMBOL_LEN_MAX, SYMBOL_UTF8_CHARS};
use lazy_static::lazy_static;
use rand::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::process::Command;

fn cleos() -> Command {
    let mut cmd = Command::new("docker");
    cmd.args(&[
        "exec",
        "eosio-rust_keosd_1",
        "cleos",
        "--url",
        "http://nodeosd:8888",
        "--wallet-url",
        "http://127.0.0.1:8900",
    ]);
    cmd
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?P<bytes>\d+) bytes  (?P<time>\d+) us")
            .expect("bad regex");
}

#[derive(Debug)]
struct Case {
    contract: String,
    action: String,
    data: String,
    scope: String,
}

#[derive(Debug)]
struct Run {
    action: String,
    bytes: u64,
    time: u64,
}

impl Case {
    fn new(contract: &str, action: &str, data: &str, scope: &str) -> Self {
        Self {
            contract: contract.into(),
            action: action.into(),
            data: data.into(),
            scope: scope.into(),
        }
    }
    fn exec(&self) -> Run {
        let out = cleos()
            .args(&[
                "push",
                "action",
                &self.contract,
                &self.action,
                &self.data,
                "-p",
                &self.scope,
            ])
            .output()
            .expect("failed to execute process");
        let stderr = String::from_utf8_lossy(&out.stderr);
        let caps = match RE.captures(&stderr) {
            Some(caps) => caps,
            None => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                panic!("No regex matches found, something probably went wrong. stderr:\n{}\n\nstdout:\n{}", stderr, stdout);
            }
        };
        let bytes = &caps["bytes"].parse::<u64>().expect("bytes is not a u64");
        let time = &caps["time"].parse::<u64>().expect("time is not a u64");
        Run {
            action: self.action.clone(),
            bytes: *bytes,
            time: *time,
        }
    }
}

fn eosio_token_tests(contract: &str, symbol: &str) -> Vec<Run> {
    let create = Case::new(
        contract,
        "create",
        &format!("[\"alice\",\"1000.00 {}\"]", symbol),
        &format!("{}@active", contract),
    )
    .exec();
    let issue = Case::new(
        contract,
        "issue",
        &format!("[\"bob\",\"1.00 {}\",\"here you go\"]", symbol),
        "alice@active",
    )
    .exec();
    let transfer = Case::new(
        contract,
        "transfer",
        &format!("[\"bob\",\"alice\",\"1.00 {}\",\"here you go\"]", symbol),
        "bob@active",
    )
    .exec();
    let retire = Case::new(
        contract,
        "retire",
        &format!("[\"1.00 {}\",\"retire\"]", symbol),
        "alice@active",
    )
    .exec();
    let close = Case::new(
        contract,
        "close",
        &format!("[\"alice\",\"2,{}\"]", symbol),
        "alice@active",
    )
    .exec();
    let open = Case::new(
        contract,
        "open",
        &format!("[\"alice\",\"2,{}\",\"alice\"]", symbol),
        "alice@active",
    )
    .exec();
    vec![create, issue, transfer, retire, close, open]
}

const WARMUP_THRESHOLD: usize = 20;

fn main() {
    println!("Running benchmarks...");
    let mut rng = rand::thread_rng();
    let mut rust_runs = Vec::new();
    let mut cpp_runs = Vec::new();

    println!("Warming up for {} iterations...", WARMUP_THRESHOLD);
    for i in 0..500 {
        if i % 5 == 0 {
            println!("Iteration #{}", i);
        }
        if i == WARMUP_THRESHOLD {
            println!("Done warming up! Saving results...");
        }

        let symbol: String = (0..SYMBOL_LEN_MAX)
            .map(|_| {
                let idx = rng.gen_range(0, SYMBOL_UTF8_CHARS.len());
                char::from(unsafe { *SYMBOL_UTF8_CHARS.get_unchecked(idx) })
            })
            .collect();
        let mut r = eosio_token_tests("eosio.token", &symbol);
        let mut c = eosio_token_tests("eosiotkncpp", &symbol);

        if i >= WARMUP_THRESHOLD {
            rust_runs.append(&mut r);
            cpp_runs.append(&mut c);
        }
    }

    let mut rust_runs_map: HashMap<&str, Vec<u64>> = HashMap::new();
    for run in &rust_runs {
        match rust_runs_map.get_mut(run.action.as_str()) {
            Some(r) => r.push(run.time),
            None => {
                rust_runs_map.insert(run.action.as_str(), vec![run.time]);
            }
        }
    }

    let mut cpp_runs_map: HashMap<&str, Vec<u64>> = HashMap::new();
    for run in &cpp_runs {
        match cpp_runs_map.get_mut(run.action.as_str()) {
            Some(r) => r.push(run.time),
            None => {
                cpp_runs_map.insert(run.action.as_str(), vec![run.time]);
            }
        }
    }

    for (k, v) in &rust_runs_map {
        let rs_sum: u64 = v.iter().sum();
        let rs_len = v.len();
        let rs_mean = rs_sum / (rs_len as u64);
        let rs_max = v.iter().max().unwrap();
        let rs_min = v.iter().min().unwrap();

        let cpp_runs = cpp_runs_map.get(k).unwrap();
        let cpp_sum: u64 = cpp_runs.iter().sum();
        let cpp_len = cpp_runs.len();
        let cpp_mean = cpp_sum / (cpp_len as u64);
        let cpp_max = cpp_runs.iter().max().unwrap();
        let cpp_min = cpp_runs.iter().min().unwrap();

        let mean_diff = (rs_mean as f64) / (cpp_mean as f64) * 100.0 - 100.0;
        let min_diff = (*rs_min as f64) / (*cpp_min as f64) * 100.0 - 100.0;
        let max_diff = (*rs_max as f64) / (*cpp_max as f64) * 100.0 - 100.0;

        println!(
            r#"Results for action '{}':
Mean: Rust = {}, C++ = {} ({:.1}% difference)
Min: Rust = {}, C++ = {} ({:.1}% difference)
Max: Rust = {}, C++ = {} ({:.1}% difference)
"#,
            k,
            rs_mean,
            cpp_mean,
            mean_diff,
            rs_min,
            cpp_min,
            min_diff,
            rs_max,
            cpp_max,
            max_diff
        );
    }
}
