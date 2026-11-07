# Style Guide — Writing under the Dual Anchors

This document turns the philosophy of Project Tat Tvam Asi into concrete coding rules.

## 1. The Law (贾战英 / Zhanying)

**Principle**: Everything that is born must have a clear owner and a determined end.

### Rules

- Prefer **ownership transfer** over `.clone()`.
- Avoid shared mutable state (`Rc<RefCell<_>>`, `Arc<Mutex<_>>`) unless the domain truly requires it.
- Make `Drop` meaningful when a resource holds external handles.
- Do not leave “temporary” values that outlive their logical scope.

### Smell

```rust
// Avoid
let data2 = data.clone();
process(data2);

// Prefer
process(data); // move
```

## 2. The Vision (贾大眼 / Dayan)

**Principle**: Every state and every branch must be named. No hidden unknowns.

### Rules

- Prefer `enum` + exhaustive `match` over `bool` flags and nested `if let`.
- Avoid `_` wildcards that silently ignore new variants.
- Avoid `unwrap` / `expect` in non-test code. Handle the error or propagate it.
- When a value can be absent, model it explicitly (`Option`, custom enum), then match it.

### Smell

```rust
// Avoid
if let Some(x) = maybe {
    // ...
}

// Prefer when all cases matter
match maybe {
    Some(x) => { /* ... */ }
    None => { /* explicit handling */ }
}
```

## 3. Śūnyatā (Zero-Cost)

**Principle**: High-level structure must not impose runtime tax.

### Rules

- Prefer static dispatch (generics + traits) over trait objects when performance matters.
- Use zero-sized types and `PhantomData` for type-level information.
- Iterator adapters and small newtypes should be `#[inline]`-friendly and allocation-free in the hot path.

## 4. Recommended Patterns from this Crate

| Pattern            | Module     | Use when                                      |
|--------------------|------------|-----------------------------------------------|
| `Resource`         | `law`      | You own bytes / handles that must be released |
| `Essence` + `Form` | `essence`  | You want name/form separation                 |
| `State` / `Phase`  | `vision` / `process` | Closed state machines                |
| `SacredMap`        | `sunyata`  | Zero-cost transformation pipelines            |
| `Phoneme<C>`       | `mantra`   | Type-level classification                     |
| `SacredProcess`    | `process`  | A full observe→act→update cycle               |

## 5. Checklist before merging

- [ ] No unnecessary `.clone()`
- [ ] No `unwrap` / `expect` outside tests
- [ ] All `match` on crate enums are exhaustive (no silent `_`)
- [ ] New public types have a clear ownership story
- [ ] Hot-path abstractions are zero-cost or documented as such

Tat Tvam Asi.
