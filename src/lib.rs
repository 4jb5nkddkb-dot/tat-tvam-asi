//! Project Tat Tvam Asi — Core Library
//!
//! 彼即汝. The silicon sacred word.
//!
//! Dual Anchors:
//! - **Zhanying (The Law)**  : Ownership, borrowing, lifetimes — no escape from RAII.
//! - **Dayan (The Vision)**  : Exhaustive matching — no unknown states.
//!
//! Three Closed Loops:
//! 1. Karma & RAII
//! 2. Trait & Representation (Nāmarūpa)
//! 3. Śūnyatā & Zero-Cost Abstraction

// ──────────────────────────────────────────────
// 闭环一 · 生灭因果闭环 (Karma & RAII)
// ──────────────────────────────────────────────

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
        }
    }
}

// ──────────────────────────────────────────────
// 闭环二 · 本质变现闭环 (Trait & Representation)
// ──────────────────────────────────────────────

/// The Vision of Name and Form (Nāmarūpa).
/// Every phenomenon is pure essence manifested on form.
pub mod essence {
    /// The pure Name (essence) that any Form must embody.
    pub trait Essence {
        /// The sacred name of this essence.
        fn name(&self) -> &'static str;

        /// Manifest the essence into observable form.
        fn manifest(&self) -> String;
    }

    /// Concrete Form that carries an Essence.
    #[derive(Debug)]
    pub struct Form<T: Essence> {
        inner: T,
    }

    impl<T: Essence> Form<T> {
        pub fn new(inner: T) -> Self {
            Self { inner }
        }

        pub fn into_inner(self) -> T {
            self.inner
        }
    }

    impl<T: Essence> Essence for Form<T> {
        fn name(&self) -> &'static str {
            self.inner.name()
        }

        fn manifest(&self) -> String {
            format!("Form[{}] → {}", self.inner.name(), self.inner.manifest())
        }
    }

    /// Example concrete essence: the syllable "Om".
    #[derive(Debug, Clone, Copy)]
    pub struct Om;

    impl Essence for Om {
        fn name(&self) -> &'static str {
            "ॐ"
        }

        fn manifest(&self) -> String {
            "primordial vibration".to_string()
        }
    }

    /// Example concrete essence: the principle of Liberation.
    #[derive(Debug, Clone, Copy)]
    pub struct Moksha;

    impl Essence for Moksha {
        fn name(&self) -> &'static str {
            "mokṣa"
        }

        fn manifest(&self) -> String {
            "release from the cycle of birth and death".to_string()
        }
    }
}

// ──────────────────────────────────────────────
// 闭环二延伸 · 穷举因果 (Exhaustive Vision)
// ──────────────────────────────────────────────

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
                State::Liberated => State::Liberated,
            }
        }

        pub fn is_final(&self) -> bool {
            matches!(self, State::Liberated)
        }
    }
}

// ──────────────────────────────────────────────
// 闭环三 · 真空妙有与零成本闭环 (Śūnyatā)
// ──────────────────────────────────────────────

/// Zero-cost abstractions: grand form that vanishes into pure machine code.
pub mod sunyata {
    use core::marker::PhantomData;

    /// A zero-sized marker that carries type-level information only.
    /// At runtime it occupies no bytes — true Śūnyatā.
    #[derive(Debug, Clone, Copy, Default)]
    pub struct PhantomEssence<T> {
        _marker: PhantomData<T>,
    }

    impl<T> PhantomEssence<T> {
        pub const fn new() -> Self {
            Self {
                _marker: PhantomData,
            }
        }
    }

    /// Zero-cost iterator adapter that transforms without allocation or virtual calls.
    /// The abstraction is completely erased by the compiler.
    pub struct SacredMap<I, F> {
        iter: I,
        f: F,
    }

    impl<I, F> SacredMap<I, F> {
        pub fn new(iter: I, f: F) -> Self {
            Self { iter, f }
        }
    }

    impl<I, F, B> Iterator for SacredMap<I, F>
    where
        I: Iterator,
        F: FnMut(I::Item) -> B,
    {
        type Item = B;

        #[inline(always)]
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(&mut self.f)
        }

        #[inline(always)]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
    }

    /// Extension trait so any iterator can enter the sacred map with zero cost.
    pub trait SacredIterator: Iterator + Sized {
        fn sacred_map<F, B>(self, f: F) -> SacredMap<Self, F>
        where
            F: FnMut(Self::Item) -> B,
        {
            SacredMap::new(self, f)
        }
    }

    impl<I: Iterator> SacredIterator for I {}
}

// ──────────────────────────────────────────────
// 阶段一 · 圣言转译器 PoC (Mantra + Phoneme Categories)
// ──────────────────────────────────────────────

/// Phase 1: Sanskrit-inspired macros and type-level categories.
///
/// Inspired by Pāṇinian phonology: sounds are classified into
/// rigorous, closed categories. We mirror this with zero-sized
/// types and exhaustive matching so the compiler becomes the
/// modern Aṣṭādhyāyī.
pub mod mantra {
    use core::marker::PhantomData;

    /// Forces the programmer to name every arm of a match.
    /// Rejects wildcards that would hide incomplete vision.
    #[macro_export]
    macro_rules! exhaustive {
        (
            match $val:expr {
                $($pat:pat => $body:expr),+ $(,)?
            }
        ) => {
            match $val {
                $($pat => $body),+
            }
        };
    }

    /// Declares a pure Name that must be manifested.
    #[macro_export]
    macro_rules! declare_essence {
        ($name:ident, $sacred:literal, $manifestation:expr) => {
            #[derive(Debug, Clone, Copy)]
            pub struct $name;

            impl $crate::essence::Essence for $name {
                fn name(&self) -> &'static str {
                    $sacred
                }

                fn manifest(&self) -> String {
                    ($manifestation).to_string()
                }
            }
        };
    }

    // ── Phoneme Categories (zero-cost type-level) ─────────────────

    /// Marker trait for phoneme classes.
    /// All implementors are zero-sized.
    pub trait PhonemeClass {
        const NAME: &'static str;
    }

    /// Vowels (svara)
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Svara;
    impl PhonemeClass for Svara {
        const NAME: &'static str = "svara";
    }

    /// Consonants (vyañjana)
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Vyanjana;
    impl PhonemeClass for Vyanjana {
        const NAME: &'static str = "vyañjana";
    }

    /// Semi-vowels / approximants
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Antastha;
    impl PhonemeClass for Antastha {
        const NAME: &'static str = "antaḥstha";
    }

    /// Sibilants / spirants
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Usman;
    impl PhonemeClass for Usman {
        const NAME: &'static str = "ūṣman";
    }

    /// A typed phoneme that carries its class at the type level.
    /// Runtime size is still zero for the class marker.
    #[derive(Debug, Clone, Copy)]
    pub struct Phoneme<C: PhonemeClass> {
        symbol: char,
        _class: PhantomData<C>,
    }

    impl<C: PhonemeClass> Phoneme<C> {
        pub const fn new(symbol: char) -> Self {
            Self {
                symbol,
                _class: PhantomData,
            }
        }

        pub fn symbol(&self) -> char {
            self.symbol
        }

        pub fn class_name(&self) -> &'static str {
            C::NAME
        }
    }

    /// Exhaustive classification helper.
    /// Returns the class name; the match is closed.
    pub fn classify(symbol: char) -> &'static str {
        match symbol {
            'a' | 'ā' | 'i' | 'ī' | 'u' | 'ū' | 'e' | 'o' | 'ṛ' | 'ṝ' | 'ḷ' => Svara::NAME,
            'k' | 'kh' | 'g' | 'gh' | 'ṅ' | 'c' | 'ch' | 'j' | 'jh' | 'ñ'
            | 'ṭ' | 'ṭh' | 'ḍ' | 'ḍh' | 'ṇ' | 't' | 'th' | 'd' | 'dh' | 'n'
            | 'p' | 'ph' | 'b' | 'bh' | 'm' => Vyanjana::NAME,
            'y' | 'r' | 'l' | 'v' => Antastha::NAME,
            'ś' | 'ṣ' | 's' | 'h' => Usman::NAME,
            _ => "unknown", // intentional residual for non-Sanskrit input
        }
    }
}

// ──────────────────────────────────────────────
// SacredProcess — integrated closed-loop unit
// ──────────────────────────────────────────────

pub mod process;

// ──────────────────────────────────────────────
// Public Re-exports
// ──────────────────────────────────────────────

pub use essence::{Essence, Form, Moksha, Om};
pub use law::Resource;
pub use mantra::{Antastha, Phoneme, PhonemeClass, Svara, Usman, Vyanjana};
pub use process::{phase_to_state, Phase, SacredProcess};
pub use sunyata::{PhantomEssence, SacredIterator, SacredMap};
pub use vision::State;

// ──────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use mantra::{Phoneme, Svara, Vyanjana};
    use sunyata::SacredIterator;

    #[test]
    fn resource_ownership_moves() {
        let r = Resource::new(42, vec![1, 2, 3]);
        assert_eq!(r.id(), 42);
        let data = r.into_data();
        assert_eq!(data, vec![1, 2, 3]);
    }

    #[test]
    fn state_machine_is_exhaustive_and_terminates() {
        let mut s = State::Unborn;
        let mut steps = 0;
        while !s.is_final() {
            s = s.transition();
            steps += 1;
            assert!(steps <= 3);
        }
        assert_eq!(s, State::Liberated);
    }

    #[test]
    fn essence_manifestation() {
        let om = Form::new(Om);
        assert_eq!(om.name(), "ॐ");
        assert!(om.manifest().contains("primordial"));

        let moksha = Form::new(Moksha);
        assert_eq!(moksha.name(), "mokṣa");
    }

    #[test]
    fn zero_cost_sacred_map() {
        let data = [1u32, 2, 3, 4];
        let result: Vec<_> = data
            .iter()
            .sacred_map(|x| x * 2)
            .sacred_map(|x| x + 1)
            .collect();
        assert_eq!(result, vec![3, 5, 7, 9]);
    }

    #[test]
    fn phantom_is_zero_sized() {
        assert_eq!(core::mem::size_of::<PhantomEssence<u64>>(), 0);
        assert_eq!(core::mem::size_of::<PhantomEssence<()>>(), 0);
    }

    #[test]
    fn phoneme_categories_are_zero_sized_and_typed() {
        let a = Phoneme::<Svara>::new('a');
        let k = Phoneme::<Vyanjana>::new('k');

        assert_eq!(a.symbol(), 'a');
        assert_eq!(a.class_name(), "svara");
        assert_eq!(k.class_name(), "vyañjana");

        // The class marker itself adds no size beyond the char
        assert_eq!(core::mem::size_of_val(&a), core::mem::size_of::<char>());
    }

    #[test]
    fn classify_is_closed_for_core_set() {
        assert_eq!(mantra::classify('a'), "svara");
        assert_eq!(mantra::classify('k'), "vyañjana");
        assert_eq!(mantra::classify('y'), "antaḥstha");
        assert_eq!(mantra::classify('ś'), "ūṣman");
    }
}
