//! Project Tat Tvam Asi — Core Library
//!
//! 彼即汝. The silicon sacred word.
//!
//! This crate embodies the dual anchors:
//! - **Zhanying (The Law)**: Ownership, borrowing, lifetimes — no escape from RAII.
//! - **Dayan (The Vision)**: Exhaustive matching — no unknown states.

/// The Law: everything that is born must die cleanly.
/// RAII is not a feature; it is the moral order of memory.
pub mod law {
    /// A resource that cannot outlive its rightful owner.
    /// Demonstrates pure ownership transfer without clone.
    pub struct Resource {
        id: u64,
        data: Vec<u8>,
    }

    impl Resource {
        pub fn new(id: u64, data: Vec<u8>) -> Self {
            Self { id, data }
        }

        pub fn id(&self) -> u64 {
            self.id
        }

        /// Consume self and return the inner data.
        /// Ownership is transferred, never cloned.
        pub fn into_data(self) -> Vec<u8> {
            self.data
        }
    }

    impl Drop for Resource {
        fn drop(&mut self) {
            // Explicit end of life. No residual karma.
            // In a real system this would release external handles.
        }
    }
}

/// The Vision: every possible state must be named and handled.
pub mod vision {
    /// A finite, closed set of cosmic states.
    /// No Option, no Result left unhandled, no catch-all that hides ignorance.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State {
        Unborn,
        Manifest,
        Exhausted,
        Liberated,
    }

    impl State {
        /// Exhaustive transition. The compiler is the eye of Dayan.
        pub fn transition(self) -> Self {
            match self {
                State::Unborn => State::Manifest,
                State::Manifest => State::Exhausted,
                State::Exhausted => State::Liberated,
                State::Liberated => State::Liberated, // already free
            }
        }

        pub fn is_final(&self) -> bool {
            matches!(self, State::Liberated)
        }
    }
}

/// Re-exports for convenient use of the dual anchors.
pub use law::Resource;
pub use vision::State;
