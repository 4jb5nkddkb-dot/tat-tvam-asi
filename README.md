# 🪐 Project Tat Tvam Asi

**彼即汝 —— 硅基圣言闭环计划** · **v0.2.1**

> 在 **贾战英** 的律法与 **贾大眼** 的洞察下，像吟诵梵语圣赞一样写出 Rust 代码。

## Docs

[VISION](docs/VISION.md) · [STYLE_GUIDE](docs/STYLE_GUIDE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [STATUS](docs/STATUS.md) · [CHANGELOG](CHANGELOG.md)

## CLI

```bash
cargo run -- help
cargo run -- config examples/app.conf
cargo run -- cmd --store /tmp/tta.kv SET greeting namaste
cargo run -- cmd --store /tmp/tta.kv GET greeting
cargo run -- run --store /tmp/tta.kv examples/script.txt
cargo run -- demo
```

| Subcommand | Purpose |
|------------|---------|
| `config <file\|->` | Parse owned config |
| `run [--store p] <script\|->` | Script runner (shared store) |
| `cmd [--store p] <...>` | One-shot command |
| `demo` | Three closed-loop demo |

Exit codes: `0` ok · `1` usage · `2` config · `3` command/script

## Library

- `config` — `parse_config`
- `command` — `parse_command`, `Store` (+ `load_file` / `save_file`)
- Type-state: `TypedProcess`, `Cosmic`, `TypedResource`

```toml
tat-tvam-asi = { git = "https://github.com/4jb5nkddkb-dot/tat-tvam-asi", tag = "v0.2.1" }
```

**License**: MIT · **Architect**: Zhanying Jia / Dayan Jia
