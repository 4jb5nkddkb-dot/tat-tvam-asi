# Changelog

All notable changes to **tat-tvam-asi** are documented here.

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project follows [Semantic Versioning](https://semver.org/).

## [0.2.1] — 2026-11-08

CLI productization and optional store persistence.

### Added

- **CLI subcommands**: `help`, `config`, `run`, `cmd`, `demo`
- **Exit codes**: `0` ok · `1` usage · `2` config · `3` command/script
- **`--store <path>`** for `cmd` / `run` — simple `key=value` file persistence
- `Store::load_file` / `Store::save_file` / `Store::from_kv_text`
- Sample files: `examples/app.conf`, `examples/script.txt`

### Notes

- Still zero runtime dependencies beyond std.
- Tag `v0.2.1` when publishing the GitHub release.

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
- **Features**: `std` (default), `alloc`, core-only
- **Examples / bench / CI / docs**

## [0.1.0] — 2026-11-07

Initial public skeleton.

[0.2.1]: https://github.com/4jb5nkddkb-dot/tat-tvam-asi/releases/tag/v0.2.1
[0.2.0]: https://github.com/4jb5nkddkb-dot/tat-tvam-asi/releases/tag/v0.2.0
[0.1.0]: https://github.com/4jb5nkddkb-dot/tat-tvam-asi/releases/tag/v0.1.0
