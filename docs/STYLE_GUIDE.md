# Style Guide

规则服务于可检验的约束，而非任何个人或教义中心。

## 1. Ownership（缘起可见）

**原则**：有生则有所属与所终；尽量不假设隐式常住。

### Rules

- Prefer ownership transfer over `.clone()`.
- Avoid shared mutable state (`Rc<RefCell<_>>`, `Arc<Mutex<_>>`) unless the domain truly requires it.
- Make `Drop` meaningful when a resource holds external handles.
- Do not leave temporary values that outlive their logical scope.

### Smell

```rust
// Avoid
let data2 = data.clone();
process(data2);

// Prefer
process(data); // move
```

## 2. Exhaustive naming（不留未名）

**原则**：状态与分支尽量被命名；静默忽略也是一种立场。

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

## 3. Zero-cost where it matters（形式不必有自性负担）

**原则**：高层结构可以存在，但不必然在热路径留下固定代价。

### Rules

- Prefer static dispatch (generics + traits) over trait objects when performance matters.
- Use zero-sized types and `PhantomData` for type-level information.
- Iterator adapters and small newtypes should be `#[inline]`-friendly and allocation-free in the hot path.

## 4. Type-state vs runtime state

| Need | Prefer | Why |
|------|--------|-----|
| Sequence must be impossible to violate | **Type-state** (`TypedProcess`, `Cosmic`, `TypedResource`) | Wrong order fails at compile time |
| Logging, serialization, dynamic step loops | **Runtime enum** (`Phase`, `State`, `SacredProcess`) | Values can be stored, printed, matched in data |
| One-shot owned payload with clear end | `TypedResource` / `Resource::into_data` | Consumption is visible |
| Demo / teaching loop | `SacredProcess` | Simple to step and print |

**Rule of thumb**: if an illegal transition would be a logic bug, encode it in the type system. If you only need observability, keep the runtime enum.

## 5. Patterns in this crate

| Pattern | Module | Use when |
|---------|--------|----------|
| `Resource` | `law` | Simple owned bytes / handles |
| `TypedResource` | `typed_law` | Consume-once lifecycle |
| `Essence` + `Form` | `essence` | Name / form separation (provisional) |
| `State` | `vision` | Runtime state machine |
| `Cosmic<S>` | `typed_vision` | Compile-time path |
| `SacredProcess` | `process` | Dynamic observe→act→update loop |
| `TypedProcess` | `typed_process` | Same sequence, compile-time order |
| `SacredMap` | `sunyata` | Zero-cost transformation pipelines |
| `Phoneme<C>` | `mantra` | Type-level classification |

## 6. Checklist before merging

- [ ] No unnecessary `.clone()`
- [ ] No `unwrap` / `expect` outside tests
- [ ] All `match` on crate enums are exhaustive (no silent `_`)
- [ ] New public types have a clear ownership story
- [ ] Hot-path abstractions are zero-cost or documented as such
- [ ] If order matters, prefer type-state over comments / runtime asserts

Tat Tvam Asi.
