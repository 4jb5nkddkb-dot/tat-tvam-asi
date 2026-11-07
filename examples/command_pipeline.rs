//! Command pipeline — library `command` API + three type-state surfaces.
//!
//! ```bash
//! cargo run --example command_pipeline
//! ```

use tat_tvam_asi::{
    parse_command, run_cosmic_cycle, run_typed_pipeline, Command, Response, Store, TypedProcess,
    TypedResource,
};

fn main() {
    println!("🪐 Command Pipeline — library Store + type-state\n");

    let script_text = b"PING\nSET greeting namaste\nGET greeting\nSTATS\nGET missing\n";
    let resource = TypedResource::new(1, script_text.to_vec());
    println!(
        "[Law] TypedResource id={} bytes={}",
        resource.id(),
        resource.data().len()
    );
    let (res_id, bytes) = resource.consume();
    let source = String::from_utf8(bytes).expect("utf8 script");
    println!("[Law] consumed resource #{res_id}\n");

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

        let label = run_typed_pipeline(i as u64, line.to_string(), |s| s.len());
        let cosmic = run_cosmic_cycle();
        let response = store.handle(cmd);
        println!(
            "[{i}] len={label} cosmic={} <- {line:?} -> {response:?}",
            cosmic.name()
        );
    }

    let p = TypedProcess::new(99, Command::Ping);
    println!(
        "\n[Process] start phase={} cosmic={:?}",
        p.phase_name(),
        p.cosmic_state()
    );
    let p = p.advance().advance().advance();
    let (p, resp) = p.execute(|_| Response::Pong);
    let done = p.advance();
    println!(
        "[Process] done phase={} cosmic={:?} resp={resp:?}",
        done.phase_name(),
        done.cosmic_state()
    );

    println!("\nTat Tvam Asi.");
}
