# Project Status Snapshot

**Version**: 0.1.0+  
**Branch**: main

## Latest (practical turn)

| Item | Status |
|------|--------|
| `config` library module | `parse_config`, closed keys, owned `Config` |
| `command` library module | `parse_command`, `Store`, closed `Command`/`Response` |
| Examples | Thin wrappers over library APIs |
| `benches/sacred_map` | Criterion: SacredMap vs std `map` |
| Type-state core | Unchanged and still primary constraint layer |

## Run

```bash
cargo test
cargo run --example config_loop
cargo run --example command_pipeline
cargo bench --bench sacred_map
```

## Honest limits

- Bench shows *intent* (should be in the same ballpark); not a formal proof of identical codegen.
- `exhaustive!` remains thin.
- Phonology remains a PoC.

Tat Tvam Asi.
