//! Closed command protocol and owned key-value store (requires `std`).

#![cfg(feature = "std")]

use std::collections::BTreeMap;

/// Inbound commands — closed universe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Ping,
    Get { key: String },
    Set { key: String, value: String },
    Stats,
}

/// Outbound responses — also closed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Pong,
    Value(Option<String>),
    Ok,
    Stats { entries: usize },
    Error(&'static str),
}

/// Parse a single whitespace-separated command line.
pub fn parse_command(line: &str) -> Result<Command, &'static str> {
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

/// Tiny owned store. No shared mutability across tasks.
#[derive(Debug, Default)]
pub struct Store {
    map: BTreeMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn handle(&mut self, cmd: Command) -> Response {
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

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_roundtrip() {
        let mut s = Store::new();
        assert!(matches!(
            s.handle(Command::Set {
                key: "k".into(),
                value: "v".into()
            }),
            Response::Ok
        ));
        assert_eq(
            s.handle(Command::Get { key: "k".into() }),
            Response::Value(Some("v".into()))
        );
    }

    #[test]
    fn parse_ping() {
        assert_eq!(parse_command("PING").unwrap(), Command::Ping);
    }
}
