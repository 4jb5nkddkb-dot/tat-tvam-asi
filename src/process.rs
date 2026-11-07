//! SacredProcess — a minimal closed-loop unit that embodies all three principles.
//!
//! - Law (Zhanying): resources are owned and dropped cleanly
//! - Vision (Dayan): every phase is named and exhaustively matched
//! - Śūnyatā: abstractions compile to nothing extra

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
    pub fn next(self) -> Option<Self> {
        match self {
            Phase::Observe => Some(Phase::Intend),
            Phase::Intend => Some(Phase::Predict),
            Phase::Predict => Some(Phase::Execute),
            Phase::Execute => Some(Phase::Evaluate),
            Phase::Evaluate => Some(Phase::Update),
            Phase::Update => None, // cycle complete
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Phase::Update)
    }
}

/// A single unit of work that owns its input and produces an owned output.
/// No shared mutable state, no hidden lifetimes.
pub struct SacredProcess<I, O> {
    id: u64,
    input: I,
    phase: Phase,
    output: Option<O>,
}

impl<I, O> SacredProcess<I, O> {
    pub fn new(id: u64, input: I) -> Self {
        Self {
            id,
            input,
            phase: Phase::Observe,
            output: None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn is_complete(&self) -> bool {
        self.output.is_some()
    }
}

impl<I, O> SacredProcess<I, O>
where
    O: Default,
{
    /// Advance one step. Returns true if the process completed.
    pub fn step<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(&I, Phase) -> O,
    {
        if self.is_complete() {
            return true;
        }

        // Produce output only on the final actionable phase for this simple model.
        if self.phase == Phase::Execute {
            let out = f(&self.input, self.phase);
            self.output = Some(out);
        }

        match self.phase.next() {
            Some(next) => {
                self.phase = next;
                false
            }
            None => true,
        }
    }

    /// Consume the process and take the output if present.
    pub fn into_output(self) -> Option<O> {
        self.output
    }
}

/// Map a SacredProcess phase sequence into the broader cosmic State.
pub fn phase_to_state(phase: Phase) -> State {
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
            proc.step(|input, phase| {
                // Simple pure function of input + phase
                input.wrapping_add(phase as u32)
            });
            steps += 1;
        }

        assert!(proc.is_complete());
        let out = proc.into_output();
        assert!(out.is_some());
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
}
