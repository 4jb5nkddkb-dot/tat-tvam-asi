//! SacredProcess — closed-loop unit embodying all three principles.
//!
//! - Law (Zhanying): input is owned; output is taken by value
//! - Vision (Dayan): every phase is named; transitions are exhaustive
//! - Śūnyatā: no hidden allocation or virtual dispatch in the hot path

use crate::vision::State;

/// The phases of a single sacred processing cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Observe,
    Intend,
    Predict,
    Execute,
    Evaluate,
    Update,
}

impl Phase {
    /// Exhaustive next-phase transition.
    pub const fn next(self) -> Option<Self> {
        match self {
            Phase::Observe => Some(Phase::Intend),
            Phase::Intend => Some(Phase::Predict),
            Phase::Predict => Some(Phase::Execute),
            Phase::Execute => Some(Phase::Evaluate),
            Phase::Evaluate => Some(Phase::Update),
            Phase::Update => None,
        }
    }

    pub const fn is_terminal(self) -> bool {
        matches!(self, Phase::Update)
    }

    pub const fn is_execute(self) -> bool {
        matches!(self, Phase::Execute)
    }
}

/// A single unit of work that owns its input and produces an owned output.
pub struct SacredProcess<I, O> {
    id: u64,
    input: I,
    phase: Phase,
    output: Option<O>,
}

impl<I, O> SacredProcess<I, O> {
    pub const fn new(id: u64, input: I) -> Self {
        Self {
            id,
            input,
            phase: Phase::Observe,
            output: None,
        }
    }

    pub const fn id(&self) -> u64 {
        self.id
    }

    pub const fn phase(&self) -> Phase {
        self.phase
    }

    /// True when the process has reached the terminal phase.
    pub const fn is_complete(&self) -> bool {
        self.phase.is_terminal()
    }

    /// Advance one step.
    ///
    /// The provided function is called exactly once, on the Execute phase.
    /// Returns `true` if the process is complete after this call.
    pub fn step<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(&I, Phase) -> O,
    {
        if self.is_complete() {
            return true;
        }

        if self.phase.is_execute() && self.output.is_none() {
            self.output = Some(f(&self.input, self.phase));
        }

        match self.phase.next() {
            Some(next) => {
                self.phase = next;
                self.is_complete()
            }
            None => true,
        }
    }

    /// Consume the process and take the output if it was produced.
    pub fn into_output(self) -> Option<O> {
        self.output
    }

    /// Borrow the output if already produced.
    pub fn output(&self) -> Option<&O> {
        self.output.as_ref()
    }
}

/// Map a SacredProcess phase into the broader cosmic State.
pub const fn phase_to_state(phase: Phase) -> State {
    match phase {
        Phase::Observe | Phase::Intend => State::Unborn,
        Phase::Predict | Phase::Execute => State::Manifest,
        Phase::Evaluate => State::Exhausted,
        Phase::Update => State::Liberated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_runs_to_completion() {
        let mut proc = SacredProcess::new(1, 42u32);
        let mut steps = 0;
        while !proc.is_complete() && steps < 10 {
            proc.step(|input, phase| input.wrapping_add(phase as u32));
            steps += 1;
        }
        assert!(proc.is_complete());
        assert!(proc.into_output().is_some());
    }

    #[test]
    fn phase_transitions_are_exhaustive() {
        let mut phase = Phase::Observe;
        let mut count = 0;
        while let Some(next) = phase.next() {
            phase = next;
            count += 1;
            assert!(count <= 5);
        }
        assert!(phase.is_terminal());
    }

    #[test]
    fn phase_to_state_mapping() {
        assert_eq!(phase_to_state(Phase::Observe), State::Unborn);
        assert_eq!(phase_to_state(Phase::Execute), State::Manifest);
        assert_eq!(phase_to_state(Phase::Update), State::Liberated);
    }

    #[test]
    fn output_produced_exactly_once() {
        let mut proc = SacredProcess::new(7, 100u32);
        let mut produced = 0u32;
        while !proc.is_complete() {
            proc.step(|input, phase| {
                produced += 1;
                input + phase as u32
            });
        }
        assert_eq!(produced, 1);
        assert!(proc.into_output().is_some());
    }
}
