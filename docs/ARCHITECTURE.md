# Architecture Overview — Project Tat Tvam Asi

## Module Map

```
tat_tvam_asi
├── law            → Karma & RAII (alloc)
├── typed_law      → Type-state resource lifecycle (alloc)
├── vision         → Runtime cosmic State enum
├── typed_vision   → Type-state cosmic path
├── essence        → Trait & Form / Nāmarūpa (alloc)
├── sunyata        → Zero-cost adapters & markers
├── mantra         → Sanskrit-Rust PoC (macros + phonology)
├── process        → Runtime-phase SacredProcess
└── typed_process  → Type-state process pipeline
```

## Feature Split

| Feature | Modules |
|---------|---------|
| core (no features) | `vision`, `typed_vision`, `sunyata`, `mantra`, `process`, `typed_process` |
| `alloc` | + `law`, `typed_law`, `essence` |
| `std` (default) | full crate + binary + examples |

## Design Principles

1. **No residual karma** — ownership is explicit; consume/move preferred over clone.
2. **No unknown states** — closed enums; type-state where sequence must be forced.
3. **Zero-cost where it matters** — `PhantomData`, marker traits, iterator adapters.
4. **Compile-time over runtime** — illegal transitions should not compile.

## Type-state surfaces

| API | Sequence enforced |
|-----|-------------------|
| `TypedProcess<I, S>` | Observe → Intend → Predict → Execute → Evaluate → Update |
| `Cosmic<S>` | Unborn → Manifest → Exhausted → Liberated |
| `TypedResource<Alive>` | construct → (map) → consume once |

Runtime twins (`Phase`, `State`, `Resource`) remain for logging and simple demos.

## SacredProcess vs TypedProcess

- `SacredProcess` — dynamic phase field; good for demos and step loops.
- `TypedProcess` — phase in the type; illegal order is a compile error.

## Phase 1 (Mantra)

- Macros: `exhaustive!`, `declare_essence!` (alloc)
- Classes: `Svara`, `Vyanjana`, `Antastha`, `Usman`
- Dimensions: `Sthana`, `Prayatna`, `Ghosa`, `Prana`
- `Phoneme<C>`, `classify` / `classify_name`

## Docs

- [VISION.md](VISION.md) · [STYLE_GUIDE.md](STYLE_GUIDE.md) · [STATUS.md](STATUS.md) · [PROJECT_PLAN.md](PROJECT_PLAN.md)
