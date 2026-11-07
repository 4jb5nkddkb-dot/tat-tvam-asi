//! Type-state SacredProcess — illegal phase transitions are rejected by the type system.
//!
//! This is the stricter sibling of `process::SacredProcess`.
//! Runtime phase enums remain useful for logging; type-state encodes the Law of sequence.

use core::marker::PhantomData;

/// Marker traits / states for the processing pipeline.
pub mod state {
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Observe;
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Intend;
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Predict;
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Execute;
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Evaluate;
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Update;
}

use state::*;

/// Process that owns its input and carries the current phase in the type parameter `S`.
pub struct TypedProcess<I, S> {
    id: u64,
    input: I,
    _state: PhantomData<S>,
}

impl<I> TypedProcess<I, Observe> {
    pub const fn new(id: u64, input: I) -> Self {
        Self {
            id,
            input,
            _state: PhantomData,
        }
    }
}

impl<I, S> TypedProcess<I, S> {
    pub const fn id(&self) -> u64 {
        self.id
    }

    pub fn input(&self) -> &I {
        &self.input
    }
}

macro_rules! advance {
    ($from:ty => $to:ty) => {
        impl<I> TypedProcess<I, $from> {
            pub fn advance(self) -> TypedProcess<I, $to> {
                TypedProcess {
                    id: self.id,
                    input: self.input,
                    _state: PhantomData,
                }
            }
        }
    };
}

advance!(Observe => Intend);
advance!(Intend => Predict);
advance!(Predict => Execute);
advance!(Execute => Evaluate);
advance!(Evaluate => Update);

impl<I> TypedProcess<I, Execute> {
    /// Only available in the Execute state — Vision forbids acting too early or too late.
    pub fn execute<O, F>(self, f: F) -> (TypedProcess<I, Evaluate>, O)
    where
        F: FnOnce(&I) -> O,
    {
        let out = f(&self.input);
        (
            TypedProcess {
                id: self.id,
                input: self.input,
                _state: PhantomData,
            },
            out,
        )
    }
}

impl<I> TypedProcess<I, Update> {
    /// Terminal state: consume and recover the owned input if needed.
    pub fn into_input(self) -> I {
        self.input
    }
}

/// Run the full typed pipeline: Observe → … → Execute(f) → … → Update.
pub fn run_typed_pipeline<I, O, F>(id: u64, input: I, f: F) -> O
where
    F: FnOnce(&I) -> O,
{
    let p = TypedProcess::new(id, input);
    let p = p.advance(); // Intend
    let p = p.advance(); // Predict
    let p = p.advance(); // Execute
    let (p, out) = p.execute(f);
    let p = p.advance(); // Update
    let _done = p;
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_pipeline_produces_output() {
        let result = run_typed_pipeline(1, 10u32, |x| x * 3);
        assert_eq!(result, 30);
    }

    #[test]
    fn execute_only_on_execute_state() {
        let p = TypedProcess::new(2, 5u32)
            .advance()
            .advance()
            .advance();
        let (p, out) = p.execute(|x| x + 1);
        assert_eq!(out, 6);
        let _ = p.advance().into_input();
    }
}
