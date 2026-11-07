//! Tiny ownership-clean configuration parser (requires `std`).
//!
//! Closed key set, exhaustive presence checks, owned result — no partial configs.

#![cfg(feature = "std")]

use std::collections::BTreeMap;

/// Known configuration keys — closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfigKey {
    Host,
    Port,
    Mode,
}

impl ConfigKey {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "host" => Some(ConfigKey::Host),
            "port" => Some(ConfigKey::Port),
            "mode" => Some(ConfigKey::Mode),
            _ => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            ConfigKey::Host => "host",
            ConfigKey::Port => "port",
            ConfigKey::Mode => "mode",
        }
    }
}

/// Application mode — no stringly typing after parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Dev,
    Prod,
}

/// Owned, validated config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub mode: Mode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    UnknownKey(String),
    Missing(&'static str),
    BadPort(String),
    BadMode(String),
}

fn parse_line(line: &str) -> Result<(ConfigKey, String), ConfigError> {
    let mut parts = line.splitn(2, '=');
    let key_raw = parts.next().unwrap_or("").trim();
    let val_raw = parts.next().unwrap_or("").trim();
    match ConfigKey::parse(key_raw) {
        Some(k) => Ok((k, val_raw.to_string())),
        None => Err(ConfigError::UnknownKey(key_raw.to_string())),
    }
}

/// Parse a full config document into an owned `Config`.
pub fn parse_config(text: &str) -> Result<Config, ConfigError> {
    let mut map: BTreeMap<ConfigKey, String> = BTreeMap::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = parse_line(line)?;
        map.insert(k, v);
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_config() {
        let cfg = parse_config("host=127.0.0.1\nport=8080\nmode=prod\n").unwrap();
        assert_eq!(cfg.host, "127.0.0.1");
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.mode, Mode::Prod);
    }

    #[test]
    fn missing_host_errors() {
        let err = parse_config("port=1\nmode=dev\n").unwrap_err();
        assert_eq!(err, ConfigError::Missing("host"));
    }
}
