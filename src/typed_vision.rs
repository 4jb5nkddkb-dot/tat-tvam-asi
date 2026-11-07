//! Type-state cosmic progression — illegal jumps fail at compile time.
//!
//! Runtime twin: `vision::State`.

use core::marker::PhantomData;

use crate::vision::State;

/// Marker states for the cosmic cycle.
pub mod cos {
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Unborn;
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Manifest;
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Exhausted;
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Liberated;
}

use cos::*;

/// Type-level handle for a cosmic stage.
pub struct Cosmic<S> {
    _state: PhantomData<S>,
}

impl Cosmic<Unborn> {
    pub const fn new() -> Self {
        Self {
            _state: PhantomData,
        }
    }

    pub fn arise(self) -> Cosmic<Manifest> {
        Cosmic {
            _state: PhantomData,
        }
    }
}

impl Default for Cosmic<Unborn> {
    fn default() -> Self {
        Self::new()
    }
}

impl Cosmic<Manifest> {
    pub fn exhaust(self) -> Cosmic<Exhausted> {
        Cosmic {
            _state: PhantomData,
        }
    }
}

impl Cosmic<Exhausted> {
    pub fn liberate(self) -> Cosmic<Liberated> {
        Cosmic {
            _state: PhantomData,
        }
    }
}

impl Cosmic<Liberated> {
    pub const fn is_final(&self) -> bool {
        true
    }
}

/// Every cosmic type-state maps to the runtime `State` enum.
pub trait CosmicState {
    const NAME: &'static str;
    fn as_state() -> State;
}

impl CosmicState for Unborn {
    const NAME: &'static str = "Unborn";
    fn as_state() -> State {
        State::Unborn
    }
}
impl CosmicState for Manifest {
    const NAME: &'static str = "Manifest";
    fn as_state() -> State {
        State::Manifest
    }
}
impl CosmicState for Exhausted {
    const NAME: &'static str = "Exhausted";
    fn as_state() -> State {
        State::Exhausted
    }
}
impl CosmicState for Liberated {
    const NAME: &'static str = "Liberated";
    fn as_state() -> State {
        State::Liberated
    }
}

impl<S: CosmicState> Cosmic<S> {
    pub fn name(&self) -> &'static str {
        S::NAME
    }

    pub fn as_state(&self) -> State {
        S::as_state()
    }
}

/// Full path Unborn → Liberated in one expression.
pub fn run_cosmic_cycle() -> Cosmic<Liberated> {
    Cosmic::new().arise().exhaust().liberate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_cycle_reaches_liberation() {
        let end = run_cosmic_cycle();
        assert!(end.is_final());
        assert_eq!(end.name(), "Liberated");
        assert_eq!(end.as_state(), State::Liberated);
    }

    #[test]
    fn stepwise_names() {
        let c = Cosmic::new();
        assert_eq!(c.name(), "Unborn");
        let c = c.arise();
        assert_eq!(c.name(), "Manifest");
        let c = c.exhaust();
        assert_eq!(c.name(), "Exhausted");
        let c = c.liberate();
        assert_eq!(c.name(), "Liberated");
    }
}
