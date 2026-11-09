# tat-tvam-asi

**彼即汝** · Rust × 中观取向的实验性系统 · v0.2.1

把所有权、穷尽匹配与零成本抽象，当作对「无自性、缘起而有」的一次工程尝试。
不宣称终极真理，只提供可编译、可检验的约束。

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
| `demo` | Closed-loop demo |

Exit codes: `0` ok · `1` usage · `2` config · `3` command/script

## Library

- `config` — `parse_config`
- `command` — `parse_command`, `Store` (+ `load_file` / `save_file`)
- Type-state: `TypedProcess`, `Cosmic`, `TypedResource`

```toml
tat-tvam-asi = { git = "https://github.com/4jb5nkddkb-dot/tat-tvam-asi", tag = "v0.2.1" }
```

**License**: MIT
