//! Tat Tvam Asi — CLI front door.
//!
//! ```text
//! tat-tvam-asi help
//! tat-tvam-asi config <file|->
//! tat-tvam-asi run [--store <path>] <script|->
//! tat-tvam-asi cmd [--store <path>] <PING|GET k|SET k v|STATS>
//! tat-tvam-asi demo
//! ```
//!
//! Exit codes:
//! - 0  success
//! - 1  usage / bad arguments
//! - 2  config error
//! - 3  command / script error

use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

use tat_tvam_asi::{parse_command, parse_config, ConfigError, Mode, Response, Store};

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print_usage();
        return ExitCode::from(1);
    }

    let cmd = args.remove(0);
    match cmd.as_str() {
        "help" | "-h" | "--help" => {
            print_usage();
            ExitCode::SUCCESS
        }
        "config" => {
            let path = match args.first() {
                Some(p) => p.clone(),
                None => {
                    eprintln!("error: config requires <file|->");
                    return ExitCode::from(1);
                }
            };
            match run_config(&path) {
                Ok(()) => ExitCode::SUCCESS,
                Err(code) => ExitCode::from(code),
            }
        }
        "run" => {
            let (store_path, rest) = split_store_flag(args);
            let path = match rest.first() {
                Some(p) => p.clone(),
                None => {
                    eprintln!("error: run requires <script|->");
                    return ExitCode::from(1);
                }
            };
            match run_script(&path, store_path.as_deref()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(code) => ExitCode::from(code),
            }
        }
        "cmd" => {
            let (store_path, rest) = split_store_flag(args);
            let line = rest.join(" ");
            if line.trim().is_empty() {
                eprintln!("error: cmd requires a command line, e.g. PING or GET key");
                return ExitCode::from(1);
            }
            match run_one_cmd(&line, store_path.as_deref()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(code) => ExitCode::from(code),
            }
        }
        "demo" => {
            run_demo();
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("error: unknown subcommand '{other}'");
            print_usage();
            ExitCode::from(1)
        }
    }
}

/// Extract optional `--store <path>` from argv; returns (store, remaining).
fn split_store_flag(args: Vec<String>) -> (Option<PathBuf>, Vec<String>) {
    let mut store = None;
    let mut rest = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--store" {
            if i + 1 < args.len() {
                store = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            } else {
                eprintln!("error: --store requires a path");
                i += 1;
            }
        } else if let Some(path) = args[i].strip_prefix("--store=") {
            store = Some(PathBuf::from(path));
            i += 1;
        } else {
            rest.push(args[i].clone());
            i += 1;
        }
    }
    (store, rest)
}

fn print_usage() {
    eprintln!(
        "\
Tat Tvam Asi CLI (v{})

Usage:
  tat-tvam-asi help
  tat-tvam-asi config <file|->
  tat-tvam-asi run  [--store <path>] <script|->
  tat-tvam-asi cmd  [--store <path>] <command...>
  tat-tvam-asi demo

Options:
  --store <path>   Persist key=value store across cmd/run invocations

Exit codes:
  0  success
  1  usage / bad arguments
  2  config error
  3  command / script error

Examples:
  tat-tvam-asi config ./examples/app.conf
  tat-tvam-asi cmd --store ./data.kv SET greeting namaste
  tat-tvam-asi cmd --store ./data.kv GET greeting
  tat-tvam-asi run --store ./data.kv ./examples/script.txt
",
        env!("CARGO_PKG_VERSION")
    );
}

fn read_input(path: &str) -> Result<String, u8> {
    if path == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).map_err(|e| {
            eprintln!("error: stdin: {e}");
            1u8
        })?;
        Ok(buf)
    } else {
        fs::read_to_string(path).map_err(|e| {
            eprintln!("error: read {path}: {e}");
            1u8
        })
    }
}

fn open_store(path: Option<&std::path::Path>) -> Result<Store, u8> {
    match path {
        None => Ok(Store::new()),
        Some(p) => Store::load_file(p).map_err(|e| {
            eprintln!("error: load store {}: {e}", p.display());
            3u8
        }),
    }
}

fn save_store(store: &Store, path: Option<&std::path::Path>) -> Result<(), u8> {
    if let Some(p) = path {
        store.save_file(p).map_err(|e| {
            eprintln!("error: save store {}: {e}", p.display());
            3u8
        })?;
    }
    Ok(())
}

fn run_config(path: &str) -> Result<(), u8> {
    let text = read_input(path)?;
    match parse_config(&text) {
        Ok(cfg) => {
            let mode = match cfg.mode {
                Mode::Dev => "dev",
                Mode::Prod => "prod",
            };
            println!("host={}", cfg.host);
            println!("port={}", cfg.port);
            println!("mode={mode}");
            Ok(())
        }
        Err(e) => {
            print_config_error(&e);
            Err(2)
        }
    }
}

fn print_config_error(e: &ConfigError) {
    match e {
        ConfigError::UnknownKey(k) => eprintln!("config error: unknown key '{k}'"),
        ConfigError::Missing(k) => eprintln!("config error: missing key '{k}'"),
        ConfigError::BadPort(p) => eprintln!("config error: bad port '{p}'"),
        ConfigError::BadMode(m) => eprintln!("config error: bad mode '{m}' (use dev|prod)"),
    }
}

fn format_response(resp: &Response) -> String {
    match resp {
        Response::Pong => "PONG".into(),
        Response::Ok => "OK".into(),
        Response::Value(Some(v)) => format!("VALUE {v}"),
        Response::Value(None) => "NULL".into(),
        Response::Stats { entries } => format!("STATS {entries}"),
        Response::Error(msg) => format!("ERROR {msg}"),
    }
}

fn run_one_cmd(line: &str, store_path: Option<&std::path::Path>) -> Result<(), u8> {
    let cmd = parse_command(line).map_err(|e| {
        eprintln!("command error: {e}");
        3u8
    })?;
    let mut store = open_store(store_path)?;
    let resp = store.handle(cmd);
    println!("{}", format_response(&resp));
    if matches!(resp, Response::Error(_)) {
        return Err(3);
    }
    save_store(&store, store_path)?;
    Ok(())
}

fn run_script(path: &str, store_path: Option<&std::path::Path>) -> Result<(), u8> {
    let text = read_input(path)?;
    let mut store = open_store(store_path)?;
    let mut failed = false;

    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match parse_command(line) {
            Ok(cmd) => {
                let resp = store.handle(cmd);
                println!("{i}: {}", format_response(&resp));
                if matches!(resp, Response::Error(_)) {
                    failed = true;
                }
            }
            Err(e) => {
                eprintln!("{i}: command error: {e} ({line:?})");
                failed = true;
            }
        }
    }

    save_store(&store, store_path)?;
    if failed {
        Err(3)
    } else {
        Ok(())
    }
}

fn run_demo() {
    use tat_tvam_asi::{
        essence::{Essence, Form, Moksha, Om},
        law::Resource,
        process::{phase_to_state, SacredProcess},
        sunyata::SacredIterator,
        vision::State,
    };

    println!("Project Tat Tvam Asi — demo\n");

    {
        let resource = Resource::new(1, b"namaste".to_vec());
        println!("[Law] resource id={}", resource.id());
        let data = resource.into_data();
        println!("[Law] consumed {:?}", String::from_utf8_lossy(&data));
    }

    let om = Form::new(Om);
    let moksha = Form::new(Moksha);
    println!("[Essence] {} -> {}", om.name(), om.manifest());
    println!("[Essence] {} -> {}", moksha.name(), moksha.manifest());

    let mut state = State::Unborn;
    print!("[Vision] {:?}", state);
    while !state.is_final() {
        state = state.transition();
        print!(" -> {:?}", state);
    }
    println!();

    let numbers = [1u32, 2, 3, 4, 5];
    let transformed: Vec<_> = numbers
        .iter()
        .sacred_map(|x| x * x)
        .sacred_map(|x| x + 1)
        .collect();
    println!("[Sunyata] {:?} -> {:?}", numbers, transformed);

    let mut proc = SacredProcess::new(108, 42u32);
    while !proc.is_complete() {
        let current = proc.phase();
        proc.step(|input, phase| input.wrapping_mul(2).wrapping_add(phase as u32));
        println!(
            "[Process] {:?} -> cosmic {:?}",
            current,
            phase_to_state(current)
        );
    }
    if let Some(result) = proc.into_output() {
        println!("[Process] output={result}");
    }
    println!("\nTat Tvam Asi.");
}
