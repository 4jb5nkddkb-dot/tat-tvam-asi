# Architecture Overview

## Module Map

```
tat_tvam_asi
├── law            → Ownership / RAII (alloc)
├── typed_law      → Type-state resource lifecycle (alloc)
├── vision         → Runtime State enum
├── typed_vision   → Type-state path
├── essence        → Trait & Form (alloc)
├── sunyata        → Zero-cost adapters & markers
├── mantra         → Phonology / type markers (macros)
├── process        → Runtime-phase process
└── typed_process  → Type-state process pipeline
```

## Feature Split

| Feature | Modules |
|---------|---------|
| core (no features) | `vision`, `typed_vision`, `sunyata`, `mantra`, `process`, `typed_process` |
| `alloc` | + `law`, `typed_law`, `essence` |
| `std` (default) | full crate + binary + examples |

## Design Principles

1. **Visible lifetime** — ownership is explicit; prefer move over clone.
2. **Named states** — closed enums; type-state where sequence must be forced.
3. **Zero-cost where it matters** — `PhantomData`, marker traits, iterator adapters.
4. **Compile-time over runtime** — illegal transitions should not compile when order is a logic requirement.

## Type-state surfaces

| API | Sequence enforced |
|-----|-------------------|
| `TypedProcess<I, S>` | Observe → Intend → Predict → Execute → Evaluate → Update |
| `Cosmic<S>` | Unborn → Manifest → Exhausted → Liberated |
| `TypedResource<Alive>` | construct → (map) → consume once |

Runtime twins (`Phase`, `State`, `Resource`) remain for logging and simple demos.

## Process variants

- Runtime process — dynamic phase field; good for demos and step loops.
- Typed process — phase in the type; illegal order is a compile error.

## Mantra (phonology sketch)

- Macros: `exhaustive!`, `declare_essence!` (alloc)
- Classes: `Svara`, `Vyanjana`, `Antastha`, `Usman`
- Dimensions: `Sthana`, `Prayatna`, `Ghosa`, `Prana`
- `Phoneme<C>`, `classify` / `classify_name`

## Docs

- [VISION.md](VISION.md) · [STYLE_GUIDE.md](STYLE_GUIDE.md) · [STATUS.md](STATUS.md) · [PROJECT_PLAN.md](PROJECT_PLAN.md)
