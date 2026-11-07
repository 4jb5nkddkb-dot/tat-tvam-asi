//! Command pipeline — a more realistic closed-loop example.
//!
//! - Closed command set (Vision / exhaustive match)
//! - Owned request → owned response (Law)
//! - Type-state pipeline for the control path (stronger constraints)
//!
//! ```bash
//! cargo run --example command_pipeline
//! ```

use tat_tvam_asi::typed_process::{run_typed_pipeline, TypedProcess};

/// Inbound commands — closed universe.
#[derive(Debug, Clone)]
enum Command {
    Ping,
    Get { key: String },
    Set { key: String, value: String },
    Stats,
}

/// Outbound effects / responses — also closed.
#[derive(Debug)]
enum Response {
    Pong,
    Value(Option<String>),
    Ok,
    Stats {
        entries: usize,
    },
    #[allow(dead_code)]
    Error(&'static str),
}

/// Tiny owned store. No shared mutability across tasks.
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
    println!("🪐 Command Pipeline\n");

    let mut store = Store::new();
    let script = [
        "PING",
        "SET greeting namaste",
        "GET greeting",
        "STATS",
        "GET missing",
    ];

    for (i, line) in script.iter().enumerate() {
        let cmd = match parse_command(line) {
            Ok(c) => c,
            Err(e) => {
                println!("[{i}] parse error: {e}");
                continue;
            }
        };

        let label = run_typed_pipeline(i as u64, line.to_string(), |s| s.len());
        let response = store.handle(cmd);
        println!("[{i}] len={label} <- {line:?} -> {response:?}");
    }

    let p = TypedProcess::new(99, Command::Ping);
    let p = p.advance().advance().advance();
    let (p, resp) = p.execute(|_| Response::Pong);
    let _done = p.advance();
    println!("\nTypedProcess single-shot: {resp:?}");

    println!("\nTat Tvam Asi.");
}
