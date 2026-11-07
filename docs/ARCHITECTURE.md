# Architecture Overview — Project Tat Tvam Asi

## Module Map

```
tat_tvam_asi
├── law          → 闭环一 · Karma & RAII          (Zhanying / The Law)
├── vision       → 穷举因果                       (Dayan / The Vision)
├── essence      → 闭环二 · Trait & Representation (Nāmarūpa)
├── sunyata      → 闭环三 · Zero-Cost Abstraction  (Śūnyatā)
├── mantra       → 阶段一 · Sanskrit-Rust PoC     (macros + phoneme categories)
└── process      → Integrated SacredProcess       (all three loops in one unit)
```

## Design Principles

1. **No residual karma**  
   Ownership is explicit. Resources are moved or dropped. Clone is avoided in the core path.

2. **No unknown states**  
   Enums are closed. Matches are exhaustive. The compiler is treated as the first witness of correctness.

3. **Zero-cost where it matters**  
   Type-level information (PhantomData, marker traits) and iterator adapters are designed to erase completely.

4. **Compile-time over runtime**  
   Prefer static dispatch, const evaluation, and macro expansion over dynamic checks.

## SacredProcess

`SacredProcess<I, O>` is the smallest unit that exercises all three closed loops together:

- Owns its input (`I`) — Law
- Advances through a fully named, exhaustive `Phase` sequence — Vision
- Produces an owned output (`O`) with no hidden allocation or virtual calls in the hot path — Śūnyatā

## Phase 1 Status

- Declarative macros: `exhaustive!`, `declare_essence!`
- Type-level phoneme classes inspired by Pāṇinian categories (`Svara`, `Vyanjana`, `Antastha`, `Usman`)
- `Phoneme<C>` carries class information at the type level with zero runtime cost for the marker

## Future Directions

- More complete mapping of sandhi and morphological rules into type constraints or macros
- Lint / CI rules that reject non-exhaustive matches and unnecessary clones in this crate
- Larger examples that demonstrate the style on realistic systems code
