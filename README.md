# 🪐 Project Tat Tvam Asi

**彼即汝 —— 硅基圣言闭环计划**

> 在 **贾战英** 的律法与 **贾大眼** 的洞察下，像吟诵梵语圣赞一样写出 Rust 代码。

## Docs

- [VISION](docs/VISION.md) · [STYLE_GUIDE](docs/STYLE_GUIDE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [STATUS](docs/STATUS.md)

## Features

| Feature | Default | Contents |
|---------|---------|----------|
| `std` | yes | full library + binary |
| `alloc` | via `std` | `law`, `essence` (heap) |
| core-only | `--no-default-features` | `vision`, `sunyata`, `process`, `typed_process`, `mantra` |

## Modules

| Module | Role |
|--------|------|
| `law` | RAII ownership |
| `vision` | Exhaustive cosmic states |
| `essence` | Name / Form traits |
| `sunyata` | Zero-cost adapters |
| `mantra` | Phonology PoC + macros |
| `process` | Runtime-phase SacredProcess |
| `typed_process` | **Type-state** pipeline (illegal transitions = compile error) |

## Build & Run

```bash
cargo test
cargo run
cargo run --example closed_loops
cargo run --example phoneme_classify
cargo run --example config_loop
cargo run --example command_pipeline

# core / alloc checks
cargo check --no-default-features
cargo check --no-default-features --features alloc
```

**Chief Architect**: Zhanying Jia / Dayan Jia · **License**: MIT
