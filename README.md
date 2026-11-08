# 🪐 Project Tat Tvam Asi

**彼即汝 —— 硅基圣言闭环计划** · **v0.2.0**

> 在 **贾战英** 的律法与 **贾大眼** 的洞察下，像吟诵梵语圣赞一样写出 Rust 代码。

## Docs

[VISION](docs/VISION.md) · [STYLE_GUIDE](docs/STYLE_GUIDE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [STATUS](docs/STATUS.md) · [CHANGELOG](CHANGELOG.md)

## CLI (product front door)

```bash
cargo run -- help
cargo run -- config path/to/app.conf
cargo run -- cmd PING
cargo run -- cmd SET greeting namaste
cargo run -- run path/to/script.txt
cargo run -- demo
```

| Subcommand | Purpose | Exit on failure |
|------------|---------|-----------------|
| `config <file\|->` | Parse owned config, print `host/port/mode` | `2` |
| `run <script\|->` | Execute command script (shared `Store`) | `3` |
| `cmd <...>` | One-shot command on a fresh store | `3` |
| `demo` | Three closed-loop demonstration | — |
| `help` | Usage | — |

Exit codes: `0` ok · `1` usage · `2` config · `3` command/script

## Library APIs (`std`)

| Module | Entry points |
|--------|----------------|
| `config` | `parse_config`, `Config`, `ConfigKey`, `Mode` |
| `command` | `parse_command`, `Store`, `Command`, `Response` |

## Type-state core

| API | Enforces |
|-----|----------|
| `TypedProcess` | phase order |
| `Cosmic` | cosmic cycle order |
| `TypedResource` | consume-once resource (alloc) |

## Dependency

```toml
tat-tvam-asi = { git = "https://github.com/4jb5nkddkb-dot/tat-tvam-asi", tag = "v0.2.0" }
```

## Test / bench

```bash
cargo test
cargo bench --bench sacred_map
cargo check --no-default-features --lib
```

**Chief Architect**: Zhanying Jia / Dayan Jia · **License**: MIT
