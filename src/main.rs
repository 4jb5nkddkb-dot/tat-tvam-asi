//! Tat Tvam Asi — CLI front door.
//!
//! ```text
//! tat-tvam-asi help
//! tat-tvam-asi config <file|->
//! tat-tvam-asi run <script|->
//! tat-tvam-asi cmd <PING|GET k|SET k v|STATS>
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
use std::process::ExitCode;

use tat_tvam_asi::{
    parse_command, parse_config, ConfigError, Mode, Response, Store,
};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let cmd = match args.next() {
        Some(c) => c,
        None => {
            print_usage();
            return ExitCode::from(1);
        }
    };

    match cmd.as_str() {
        "help" | "-h" | "--help" => {
            print_usage();
            ExitCode::SUCCESS
        }
        "config" => {
            let path = match args.next() {
                Some(p) => p,
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
            let path = match args.next() {
                Some(p) => p,
                None => {
                    eprintln!("error: run requires <script|->");
                    return ExitCode::from(1);
                }
            };
            match run_script(&path) {
                Ok(()) => ExitCode::SUCCESS,
                Err(code) => ExitCode::from(code),
            }
        }
        "cmd" => {
            let line = args.collect::<Vec<_>>().join(" ");
            if line.trim().is_empty() {
                eprintln!("error: cmd requires a command line, e.g. PING or GET key");
                return ExitCode::from(1);
            }
            match run_one_cmd(&line) {
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

fn print_usage() {
    eprintln!(
        "\
Tat Tvam Asi CLI (v{})

Usage:
  tat-tvam-asi help
  tat-tvam-asi config <file|->     Parse and print owned config
  tat-tvam-asi run <script|->      Execute command script (one cmd per line)
  tat-tvam-asi cmd <command...>    Run a single command against a fresh store
  tat-tvam-asi demo                Philosophical three-loop demonstration

Exit codes:
  0  success
  1  usage / bad arguments
  2  config error
  3  command / script error

Examples:
  tat-tvam-asi config ./app.conf
  echo 'host=127.0.0.1\nport=8080\nmode=dev' | tat-tvam-asi config -
  tat-tvam-asi cmd PING
  tat-tvam-asi cmd SET greeting namaste
  tat-tvam-asi run ./script.txt
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

fn run_one_cmd(line: &str) -> Result<(), u8> {
    let cmd = parse_command(line).map_err(|e| {
        eprintln!("command error: {e}");
        3u8
    })?;
    let mut store = Store::new();
    let resp = store.handle(cmd);
    println!("{}", format_response(&resp));
    if matches!(resp, Response::Error(_)) {
        Err(3)
    } else {
        Ok(())
    }
}

fn run_script(path: &str) -> Result<(), u8> {
    let text = read_input(path)?;
    let mut store = Store::new();
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
