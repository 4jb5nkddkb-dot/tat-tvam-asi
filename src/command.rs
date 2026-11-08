//! Closed command protocol and owned key-value store (requires `std`).

#![cfg(feature = "std")]

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

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

    /// Load from a simple `key=value` text file.
    /// Missing file → empty store (not an error).
    pub fn load_file(path: &Path) -> io::Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }
        let text = fs::read_to_string(path)?;
        Ok(Self::from_kv_text(&text))
    }

    /// Parse `key=value` lines (empty / `#` comments ignored).
    pub fn from_kv_text(text: &str) -> Self {
        let mut map = BTreeMap::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                map.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
        Self { map }
    }

    /// Persist as sorted `key=value` lines.
    pub fn save_file(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let mut out = String::new();
        out.push_str("# tat-tvam-asi store\n");
        for (k, v) in &self.map {
            out.push_str(k);
            out.push('=');
            out.push_str(v);
            out.push('\n');
        }
        fs::write(path, out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
        assert_eq!(
            s.handle(Command::Get { key: "k".into() }),
            Response::Value(Some("v".into())),
        );
    }

    #[test]
    fn parse_ping() {
        assert_eq!(parse_command("PING").unwrap(), Command::Ping);
    }

    #[test]
    fn kv_text_roundtrip_logic() {
        let mut s = Store::new();
        s.handle(Command::Set {
            key: "a".into(),
            value: "1".into(),
        });
        s.handle(Command::Set {
            key: "b".into(),
            value: "two words".into(),
        });
        let text = {
            let mut out = String::new();
            for (k, v) in &s.map {
                out.push_str(k);
                out.push('=');
                out.push_str(v);
                out.push('\n');
            }
            out
        };
        let s2 = Store::from_kv_text(&text);
        assert_eq!(s2.len(), 2);
        assert_eq!(
            s2.handle(Command::Get { key: "b".into() }),
            Response::Value(Some("two words".into()))
        );
    }

    #[test]
    fn load_missing_file_is_empty() {
        let path = PathBuf::from("/tmp/tat-tvam-asi-no-such-store-file.kv");
        let _ = fs::remove_file(&path);
        let s = Store::load_file(&path).unwrap();
        assert!(s.is_empty());
    }
}
