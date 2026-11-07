# Project Status Snapshot

**Version**: 0.1.0+  
**Branch**: main

## Latest

| Item | Status |
|------|--------|
| `typed_process` | Expanded (PhaseState, map_input, execute_finish, owned pipeline) |
| `typed_vision` | New — Cosmic type-state cycle |
| `typed_law` | New — TypedResource<Alive> consume-once (alloc) |
| CI no_std job | Active (core + alloc lib checks) |
| ARCHITECTURE | Synced with typed_* modules |

## Type-state coverage

- Process pipeline → `TypedProcess`
- Cosmic cycle → `Cosmic`
- Resource lifecycle → `TypedResource` (alloc)

## Honest limits

- `exhaustive!` is still a thin helper.
- Phonology remains a PoC.
- Not every public API is type-stated (by design: runtime twins stay for demos).

Tat Tvam Asi.
