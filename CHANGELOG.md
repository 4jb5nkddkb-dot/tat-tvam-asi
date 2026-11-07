# Changelog

All notable changes to **tat-tvam-asi** are documented here.

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project follows [Semantic Versioning](https://semver.org/).

## [0.2.0] — 2026-11-07

First practical release: type-state core + reusable library APIs + measurable zero-cost intent.

### Added

- **Type-state core**
  - `TypedProcess` / `PhaseState` / `run_typed_pipeline` / `run_typed_pipeline_owned`
  - `Cosmic` / `CosmicState` / `run_cosmic_cycle`
  - `TypedResource` (alloc) — consume-once resource lifecycle
- **Practical APIs (`std`)**
  - `config`: `parse_config`, `Config`, `ConfigKey`, `Mode`, `ConfigError`
  - `command`: `parse_command`, `Store`, `Command`, `Response`
- **Features**
  - `std` (default), `alloc`, core-only via `--no-default-features`
- **Examples**
  - `closed_loops`, `phoneme_classify`, `config_loop`, `command_pipeline`
- **Bench**
  - `benches/sacred_map` — Criterion comparison vs std `Iterator::map`
- **CI**
  - fmt + test + clippy
  - dedicated no_std / alloc lib jobs
- **Docs**
  - VISION, STYLE_GUIDE (type-state selection), ARCHITECTURE, STATUS

### Notes

- `exhaustive!` is a thin helper, not a full lint.
- Phonology (`mantra`) remains a structured PoC.
- Bench validates performance *intent*, not formal codegen equality.

## [0.1.0] — 2026-11-07

Initial public skeleton: dual anchors, three closed loops, basic modules, CI bootstrap.

[0.2.0]: https://github.com/4jb5nkddkb-dot/tat-tvam-asi/releases/tag/v0.2.0
[0.1.0]: https://github.com/4jb5nkddkb-dot/tat-tvam-asi/releases/tag/v0.1.0
