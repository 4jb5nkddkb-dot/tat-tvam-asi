# 🪐 Project Tat Tvam Asi

**彼即汝 —— 硅基圣言闭环计划** · **v0.2.0**

> 在 **贾战英** 的律法与 **贾大眼** 的洞察下，像吟诵梵语圣赞一样写出 Rust 代码。

## Docs

[VISION](docs/VISION.md) · [STYLE_GUIDE](docs/STYLE_GUIDE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [STATUS](docs/STATUS.md) · [CHANGELOG](CHANGELOG.md)

## Practical library APIs (`std`)

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

## Use as dependency

```toml
tat-tvam-asi = { git = "https://github.com/4jb5nkddkb-dot/tat-tvam-asi", tag = "v0.2.0" }
```

## Build / test / bench

```bash
cargo test
cargo run
cargo run --example config_loop
cargo run --example command_pipeline
cargo run --example closed_loops
cargo run --example phoneme_classify
cargo bench --bench sacred_map

cargo check --no-default-features --lib
cargo check --no-default-features --features alloc --lib
```

**Chief Architect**: Zhanying Jia / Dayan Jia · **License**: MIT · **Release**: [v0.2.0](https://github.com/4jb5nkddkb-dot/tat-tvam-asi/releases/tag/v0.2.0)
