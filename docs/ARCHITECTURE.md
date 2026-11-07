# Architecture Overview — Project Tat Tvam Asi

## Module Map

```
tat_tvam_asi
├── law          → 闭环一 · Karma & RAII          (Zhanying / The Law)
├── vision       → 穷举因果                       (Dayan / The Vision)
├── essence      → 闭环二 · Trait & Representation (Nāmarūpa)
├── sunyata      → 闭环三 · Zero-Cost Abstraction  (Śūnyatā)
├── mantra       → 阶段一 · Sanskrit-Rust PoC     (macros + expanded phonology)
└── process      → Integrated SacredProcess       (all three loops in one unit)
```

## Design Principles

1. **No residual karma**  
   Ownership is explicit. Resources are moved or dropped. Clone is avoided in the core path.

2. **No unknown states**  
   Enums are closed. Matches are exhaustive. The compiler is the first witness of correctness.

3. **Zero-cost where it matters**  
   Type-level information (`PhantomData`, marker traits) and iterator adapters are designed to erase completely.

4. **Compile-time over runtime**  
   Prefer static dispatch, const evaluation, and macro expansion over dynamic checks.

## SacredProcess

`SacredProcess<I, O>` exercises all three closed loops:

- Owns its input (`I`) — Law
- Advances through a fully named, exhaustive `Phase` sequence — Vision
- Produces an owned output (`O`) with no hidden allocation in the hot path — Śūnyatā

## Phase 1 Status (Mantra)

- Declarative macros: `exhaustive!`, `declare_essence!`
- Primary classes: `Svara`, `Vyanjana`, `Antastha`, `Usman`
- Secondary dimensions: `Sthana`, `Prayatna`, `Ghosa`, `Prana`
- `Phoneme<C>` — type-level class marker (zero-cost)
- `classify` / `classify_name` — closed classification functions

## Documentation

- [VISION.md](VISION.md) — calibrated world-impact vision
- [STYLE_GUIDE.md](STYLE_GUIDE.md) — concrete coding rules under the dual anchors
- [PROJECT_PLAN.md](PROJECT_PLAN.md) — original project plan

## Future Directions

- Stronger linting (reject non-exhaustive matches and unnecessary clones more aggressively)
- Type-state refinements for `SacredProcess`
- Broader morphological / sandhi experiments kept strictly zero-cost
- Optional `no_std` feature flag for embedded / high-reliability targets
