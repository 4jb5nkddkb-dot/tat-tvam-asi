//! Command pipeline — three type-state surfaces in one realistic flow.
//!
//! 1. `TypedResource` (Law)     — payload owned, consumed once
//! 2. `TypedProcess`  (Process) — phase order enforced at compile time
//! 3. `Cosmic`        (Vision)  — cosmic cycle mirrored for logging
//!
//! ```bash
//! cargo run --example command_pipeline
//! ```

use tat_tvam_asi::{
    run_cosmic_cycle, run_typed_pipeline, TypedProcess, TypedResource,
};

/// Inbound commands — closed universe.
#[derive(Debug, Clone)]
enum Command {
    Ping,
    Get { key: String },
    Set { key: String, value: String },
    Stats,
}

/// Outbound responses — also closed.
#[derive(Debug)]
enum Response {
    Pong,
    Value(Option<String>),
    Ok,
    Stats { entries: usize },
    #[allow(dead_code)]
    Error(&'static str),
}

struct Store {
    map: std::collections::BTreeMap<String, String>,
}

impl Store {
    fn new() -> Self {
        Self {
            map: std::collections::BTreeMap::new(),
        }
    }

    fn handle(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::Ping => Response::Pong,
            Command::Get { key } => Response::Value(self.map.get(&key).cloned()),
            Command::Set { key, value } => {
                self.map.insert(key, value);
                Response::Ok
            }
            Command::Stats => Response::Stats {
                entries: self.map.len(),
            },
        }
    }
}

fn parse_command(line: &str) -> Result<Command, &'static str> {
    let line = line.trim();
    if line.is_empty() {
        return Err("empty");
    }
    let mut parts = line.split_whitespace();
    let head = parts.next().unwrap_or("");
    match head {
        "PING" => Ok(Command::Ping),
        "GET" => {
            let key = parts.next().ok_or("GET needs key")?.to_string();
            Ok(Command::Get { key })
        }
        "SET" => {
            let key = parts.next().ok_or("SET needs key")?.to_string();
            let value = parts.collect::<Vec<_>>().join(" ");
            if value.is_empty() {
                return Err("SET needs value");
            }
            Ok(Command::Set { key, value })
        }
        "STATS" => Ok(Command::Stats),
        _ => Err("unknown command"),
    }
}

fn main() {
    println!("🪐 Command Pipeline — three type-state surfaces\n");

    // ── Law: typed resource holds the raw script bytes, consumed once ──
    let script_text = b"PING\nSET greeting namaste\nGET greeting\nSTATS\nGET missing\n";
    let resource = TypedResource::new(1, script_text.to_vec());
    println!(
        "[Law] TypedResource id={} bytes={}",
        resource.id(),
        resource.data().len()
    );
    let (res_id, bytes) = resource.consume();
    let source = String::from_utf8(bytes).expect("utf8 script");
    println!("[Law] consumed resource #{res_id}, script owned as String\n");

    let mut store = Store::new();

    for (i, line) in source.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let cmd = match parse_command(line) {
            Ok(c) => c,
            Err(e) => {
                println!("[{i}] parse error: {e}");
                continue;
            }
        };

        // Process type-state: pure measure of the line through the pipeline.
        let label = run_typed_pipeline(i as u64, line.to_string(), |s| s.len());

        // Vision type-state: each command is treated as one cosmic breath.
        let cosmic = run_cosmic_cycle();

        let response = store.handle(cmd);
        println!(
            "[{i}] len={label} cosmic={} <- {line:?} -> {response:?}",
            cosmic.name()
        );
    }

    // Explicit TypedProcess walk (shows phase names).
    let p = TypedProcess::new(99, Command::Ping);
    println!(
        "\n[Process] start phase={} cosmic={:?}",
        p.phase_name(),
        p.cosmic_state()
    );
    let p = p.advance().advance().advance(); // → Execute
    println!("[Process] execute phase={}", p.phase_name());
    let (p, resp) = p.execute(|_| Response::Pong);
    let done = p.advance(); // → Update
    println!(
        "[Process] done phase={} cosmic={:?} resp={resp:?}",
        done.phase_name(),
        done.cosmic_state()
    );

    println!("\nTat Tvam Asi.");
}
