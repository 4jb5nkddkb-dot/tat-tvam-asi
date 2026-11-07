//! Non-toy example: parse a tiny config, own the data, handle fields exhaustively.
//!
//! Demonstrates Law (ownership), Vision (exhaustive field match), and a closed process.
//!
//! ```bash
//! cargo run --example config_loop
//! ```

use std::collections::BTreeMap;

use tat_tvam_asi::process::SacredProcess;

/// Known configuration keys — closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ConfigKey {
    Host,
    Port,
    Mode,
}

impl ConfigKey {
    fn parse(raw: &str) -> Option<Self> {
        match raw {
            "host" => Some(ConfigKey::Host),
            "port" => Some(ConfigKey::Port),
            "mode" => Some(ConfigKey::Mode),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            ConfigKey::Host => "host",
            ConfigKey::Port => "port",
            ConfigKey::Mode => "mode",
        }
    }
}

/// Application mode — closed enum, no stringly typing after parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Dev,
    Prod,
}

/// Owned, validated config. No shared mutability.
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    mode: Mode,
}

#[derive(Debug)]
enum ConfigError {
    UnknownKey(String),
    Missing(&'static str),
    BadPort(String),
    BadMode(String),
}

fn parse_line(line: &str) -> Result<(ConfigKey, String), ConfigError> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return Err(ConfigError::UnknownKey(line.to_string()));
    }
    let mut parts = line.splitn(2, '=');
    let key_raw = parts.next().unwrap_or("").trim();
    let val_raw = parts.next().unwrap_or("").trim();
    match ConfigKey::parse(key_raw) {
        Some(k) => Ok((k, val_raw.to_string())),
        None => Err(ConfigError::UnknownKey(key_raw.to_string())),
    }
}

fn build_config(text: &str) -> Result<Config, ConfigError> {
    let mut map: BTreeMap<ConfigKey, String> = BTreeMap::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = parse_line(line)?;
        map.insert(k, v);
    }

    // Exhaustive presence check for the closed key set.
    let host = match map.remove(&ConfigKey::Host) {
        Some(h) => h,
        None => return Err(ConfigError::Missing(ConfigKey::Host.as_str())),
    };
    let port_raw = match map.remove(&ConfigKey::Port) {
        Some(p) => p,
        None => return Err(ConfigError::Missing(ConfigKey::Port.as_str())),
    };
    let mode_raw = match map.remove(&ConfigKey::Mode) {
        Some(m) => m,
        None => return Err(ConfigError::Missing(ConfigKey::Mode.as_str())),
    };

    let port: u16 = match port_raw.parse() {
        Ok(p) => p,
        Err(_) => return Err(ConfigError::BadPort(port_raw)),
    };

    let mode = match mode_raw.as_str() {
        "dev" => Mode::Dev,
        "prod" => Mode::Prod,
        other => return Err(ConfigError::BadMode(other.to_string())),
    };

    Ok(Config { host, port, mode })
}

fn main() {
    println!("🪐 Config Loop — ownership-clean parse\n");

    let source = "\n# example\nhost=127.0.0.1\nport=8080\nmode=prod\n";

    // SacredProcess owns the source string and yields an owned Config.
    let mut proc = SacredProcess::new(1, source.to_string());
    while !proc.is_complete() {
        proc.step(|input, _phase| build_config(input));
    }

    match proc.into_output() {
        Some(Ok(cfg)) => {
            println!("  host = {}", cfg.host);
            println!("  port = {}", cfg.port);
            println!("  mode = {:?}", cfg.mode);
            println!("\nConfig owned and validated. No residual partial state.");
        }
        Some(Err(e)) => println!("  error: {:?}", e),
        None => println!("  process produced no output"),
    }

    println!("Tat Tvam Asi.");
}
