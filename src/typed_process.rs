//! Type-state SacredProcess — illegal phase transitions are rejected by the type system.
//!
//! Stricter sibling of `process::SacredProcess`.
//! Runtime phase enums remain useful for logging; type-state encodes sequence Law.

use core::marker::PhantomData;

use crate::process::Phase;
use crate::vision::State;

/// Marker states for the processing pipeline.
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

/// Every type-state phase has a stable name and runtime `Phase` twin.
pub trait PhaseState {
    const NAME: &'static str;
    fn as_phase() -> Phase;
    fn as_cosmic() -> State;
}

impl PhaseState for Observe {
    const NAME: &'static str = "Observe";
    fn as_phase() -> Phase {
        Phase::Observe
    }
    fn as_cosmic() -> State {
        State::Unborn
    }
}
impl PhaseState for Intend {
    const NAME: &'static str = "Intend";
    fn as_phase() -> Phase {
        Phase::Intend
    }
    fn as_cosmic() -> State {
        State::Unborn
    }
}
impl PhaseState for Predict {
    const NAME: &'static str = "Predict";
    fn as_phase() -> Phase {
        Phase::Predict
    }
    fn as_cosmic() -> State {
        State::Manifest
    }
}
impl PhaseState for Execute {
    const NAME: &'static str = "Execute";
    fn as_phase() -> Phase {
        Phase::Execute
    }
    fn as_cosmic() -> State {
        State::Manifest
    }
}
impl PhaseState for Evaluate {
    const NAME: &'static str = "Evaluate";
    fn as_phase() -> Phase {
        Phase::Evaluate
    }
    fn as_cosmic() -> State {
        State::Exhausted
    }
}
impl PhaseState for Update {
    const NAME: &'static str = "Update";
    fn as_phase() -> Phase {
        Phase::Update
    }
    fn as_cosmic() -> State {
        State::Liberated
    }
}

/// Process that owns its input; current phase is the type parameter `S`.
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

impl<I, S: PhaseState> TypedProcess<I, S> {
    pub const fn id(&self) -> u64 {
        self.id
    }

    pub fn input(&self) -> &I {
        &self.input
    }

    pub fn phase_name(&self) -> &'static str {
        S::NAME
    }

    pub fn phase(&self) -> Phase {
        S::as_phase()
    }

    pub fn cosmic_state(&self) -> State {
        S::as_cosmic()
    }

    /// Transform owned input without leaving the current phase.
    pub fn map_input<J, F>(self, f: F) -> TypedProcess<J, S>
    where
        F: FnOnce(I) -> J,
    {
        TypedProcess {
            id: self.id,
            input: f(self.input),
            _state: PhantomData,
        }
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
    /// Only available in Execute — Vision forbids acting too early or too late.
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

    /// Execute and discard intermediate Evaluate handle, jumping to Update after.
    pub fn execute_finish<O, F>(self, f: F) -> (TypedProcess<I, Update>, O)
    where
        F: FnOnce(&I) -> O,
    {
        let (eval, out) = self.execute(f);
        (eval.advance(), out)
    }
}

impl<I> TypedProcess<I, Update> {
    /// Terminal state: recover owned input.
    pub fn into_input(self) -> I {
        self.input
    }

    pub fn is_terminal(&self) -> bool {
        true
    }
}

/// Observe → … → Execute(f) → … → Update, returning only the output.
pub fn run_typed_pipeline<I, O, F>(id: u64, input: I, f: F) -> O
where
    F: FnOnce(&I) -> O,
{
    let p = TypedProcess::new(id, input);
    let p = p.advance().advance().advance(); // → Execute
    let (p, out) = p.execute(f);
    let _done = p.advance(); // → Update
    out
}

/// Same as `run_typed_pipeline`, also returning the terminal process (input still owned).
pub fn run_typed_pipeline_owned<I, O, F>(id: u64, input: I, f: F) -> (TypedProcess<I, Update>, O)
where
    F: FnOnce(&I) -> O,
{
    let p = TypedProcess::new(id, input).advance().advance().advance();
    p.execute_finish(f)
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
        let p = TypedProcess::new(2, 5u32).advance().advance().advance();
        assert_eq!(p.phase_name(), "Execute");
        let (p, out) = p.execute(|x| x + 1);
        assert_eq!(out, 6);
        assert_eq!(p.phase_name(), "Evaluate");
        let done = p.advance();
        assert!(done.is_terminal());
        assert_eq!(done.into_input(), 5);
    }

    #[test]
    fn map_input_preserves_phase() {
        let p = TypedProcess::new(3, 1u32).map_input(|x| x + 9);
        assert_eq!(p.phase_name(), "Observe");
        assert_eq!(*p.input(), 10);
    }

    #[test]
    fn execute_finish_lands_on_update() {
        let (done, out) = run_typed_pipeline_owned(4, 2u32, |x| x * 5);
        assert_eq!(out, 10);
        assert_eq!(done.phase_name(), "Update");
        assert_eq!(done.cosmic_state(), State::Liberated);
    }

    #[test]
    fn phase_state_names_are_stable() {
        assert_eq!(Observe::NAME, "Observe");
        assert_eq!(Update::as_phase(), Phase::Update);
    }
}
