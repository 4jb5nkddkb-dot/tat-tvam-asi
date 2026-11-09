# Project Status

**Version**: 0.2.1  
**Branch**: main

## CLI

| Command | Role |
|---------|------|
| `tat-tvam-asi config <file\|->` | Owned config parse |
| `tat-tvam-asi run <script\|->` | Command script runner |
| `tat-tvam-asi cmd <...>` | One-shot command |
| `tat-tvam-asi demo` | Demo |

Exit codes: `0` ok · `1` usage · `2` config · `3` command/script

## Core

- Type-state: `TypedProcess`, `Cosmic`, `TypedResource`
- Library: `config`, `command`
- CI / bench / docs

```bash
cargo run -- help
cargo run -- cmd PING
cargo test
```
