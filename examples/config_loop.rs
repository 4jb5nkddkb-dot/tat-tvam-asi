//! Config loop — thin demo over the library `config` module.
//!
//! ```bash
//! cargo run --example config_loop
//! ```

use tat_tvam_asi::process::SacredProcess;
use tat_tvam_asi::{parse_config, ConfigError};

fn main() {
    println!("🪐 Config Loop — library parse_config\n");

    let source = "\n# example\nhost=127.0.0.1\nport=8080\nmode=prod\n";

    let mut proc = SacredProcess::new(1, source.to_string());
    while !proc.is_complete() {
        proc.step(|input, _phase| parse_config(input));
    }

    match proc.into_output() {
        Some(Ok(cfg)) => {
            println!("  host = {}", cfg.host);
            println!("  port = {}", cfg.port);
            println!("  mode = {:?}", cfg.mode);
            println!("\nConfig owned and validated via library API.");
        }
        Some(Err(e)) => match e {
            ConfigError::Missing(k) => println!("  missing key: {k}"),
            other => println!("  error: {other:?}"),
        },
        None => println!("  process produced no output"),
    }

    println!("Tat Tvam Asi.");
}
