# Project Status Snapshot

**Version**: **0.2.0**  
**Branch**: main  
**Released**: 2026-11-07

## 0.2.0 baseline

| Layer | Contents |
|-------|----------|
| Type-state | `TypedProcess`, `Cosmic`, `TypedResource` |
| Practical APIs | `config`, `command` |
| Examples | 4 runnable demos |
| Bench | `sacred_map` (Criterion) |
| CI | fmt + test + clippy + no_std jobs |
| Docs | VISION / STYLE_GUIDE / ARCHITECTURE / CHANGELOG |

## Install / use

```toml
# Cargo.toml (git dependency until crates.io publish)
tat-tvam-asi = { git = "https://github.com/4jb5nkddkb-dot/tat-tvam-asi", tag = "v0.2.0" }
```

```bash
cargo test
cargo bench --bench sacred_map
```

See [CHANGELOG.md](../CHANGELOG.md) for the full delta from 0.1.0.

Tat Tvam Asi.
