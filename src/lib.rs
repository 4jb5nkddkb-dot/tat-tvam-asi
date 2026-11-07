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
pub mod law {
    /// A resource that cannot outlive its rightful owner.
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

pub mod essence {
    /// The pure Name (essence) that any Form must embody.
    pub trait Essence {
        fn name(&self) -> &'static str;
        fn manifest(&self) -> String;
    }

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
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State {
        Unborn,
        Manifest,
        Exhausted,
        Liberated,
    }

    impl State {
        pub const fn transition(self) -> Self {
            match self {
                State::Unborn => State::Manifest,
                State::Manifest => State::Exhausted,
                State::Exhausted => State::Liberated,
                State::Liberated => State::Liberated,
            }
        }

        pub const fn is_final(self) -> bool {
            matches!(self, State::Liberated)
        }
    }
}

// ──────────────────────────────────────────────
// 闭环三 · 真空妙有与零成本闭环 (Śūnyatā)
// ──────────────────────────────────────────────

pub mod sunyata {
    use core::marker::PhantomData;

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
// 阶段一 · 圣言转译器 (Mantra + Expanded Phonology)
// ──────────────────────────────────────────────

pub mod mantra {
    use core::marker::PhantomData;

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

    pub trait PhonemeClass {
        const NAME: &'static str;
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Svara;
    impl PhonemeClass for Svara {
        const NAME: &'static str = "svara";
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Vyanjana;
    impl PhonemeClass for Vyanjana {
        const NAME: &'static str = "vyañjana";
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Antastha;
    impl PhonemeClass for Antastha {
        const NAME: &'static str = "antaḥstha";
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Usman;
    impl PhonemeClass for Usman {
        const NAME: &'static str = "ūṣman";
    }

    /// Place of articulation (sthāna)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Sthana {
        Kantha,    // guttural
        Talavya,   // palatal
        Murdhanya, // retroflex
        Dantya,    // dental
        Osthya,    // labial
    }

    /// Manner (simplified prayatna)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Prayatna {
        Sparsha,
        Antastha,
        Usman,
        Svara,
    }

    /// Voicing
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Ghosa {
        Aghosa,
        Ghosavat,
    }

    /// Aspiration
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Prana {
        Alpaprana,
        Mahaprana,
    }

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

        pub const fn symbol(&self) -> char {
            self.symbol
        }

        pub fn class_name(&self) -> &'static str {
            C::NAME
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Classification {
        pub class: &'static str,
        pub sthana: Option<Sthana>,
        pub prayatna: Prayatna,
        pub ghosa: Option<Ghosa>,
        pub prana: Option<Prana>,
    }

    /// Closed classification into Pāṇinian-inspired categories.
    pub fn classify(symbol: char) -> Classification {
        match symbol {
            'a' | 'ā' | 'i' | 'ī' | 'u' | 'ū' | 'e' | 'o' | 'ṛ' | 'ṝ' | 'ḷ' => Classification {
                class: Svara::NAME,
                sthana: None,
                prayatna: Prayatna::Svara,
                ghosa: None,
                prana: None,
            },
            'k' => Classification {
                class: Vyanjana::NAME,
                sthana: Some(Sthana::Kantha),
                prayatna: Prayatna::Sparsha,
                ghosa: Some(Ghosa::Aghosa),
                prana: Some(Prana::Alpaprana),
            },
            'g' => Classification {
                class: Vyanjana::NAME,
                sthana: Some(Sthana::Kantha),
                prayatna: Prayatna::Sparsha,
                ghosa: Some(Ghosa::Ghosavat),
                prana: Some(Prana::Alpaprana),
            },
            'y' | 'r' | 'l' | 'v' => Classification {
                class: Antastha::NAME,
                sthana: None,
                prayatna: Prayatna::Antastha,
                ghosa: Some(Ghosa::Ghosavat),
                prana: Some(Prana::Alpaprana),
            },
            'ś' | 'ṣ' | 's' | 'h' => Classification {
                class: Usman::NAME,
                sthana: None,
                prayatna: Prayatna::Usman,
                ghosa: Some(Ghosa::Aghosa),
                prana: Some(Prana::Mahaprana),
            },
            _ if symbol.is_ascii_alphabetic() => Classification {
                class: Vyanjana::NAME,
                sthana: None,
                prayatna: Prayatna::Sparsha,
                ghosa: None,
                prana: None,
            },
            _ => Classification {
                class: "unknown",
                sthana: None,
                prayatna: Prayatna::Sparsha,
                ghosa: None,
                prana: None,
            },
        }
    }

    pub fn classify_name(symbol: char) -> &'static str {
        classify(symbol).class
    }
}

// ──────────────────────────────────────────────
// SacredProcess
// ──────────────────────────────────────────────

pub mod process;

// ──────────────────────────────────────────────
// Public Re-exports
// ──────────────────────────────────────────────

pub use essence::{Essence, Form, Moksha, Om};
pub use law::Resource;
pub use mantra::{
    classify, classify_name, Antastha, Classification, Ghosa, Phoneme, PhonemeClass, Prana,
    Prayatna, Sthana, Svara, Usman, Vyanjana,
};
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
    }

    #[test]
    fn phoneme_categories_are_typed() {
        let a = Phoneme::<Svara>::new('a');
        let k = Phoneme::<Vyanjana>::new('k');
        assert_eq!(a.class_name(), "svara");
        assert_eq!(k.class_name(), "vyañjana");
    }

    #[test]
    fn classify_returns_rich_information() {
        let c = classify('k');
        assert_eq!(c.class, "vyañjana");
        assert_eq!(c.sthana, Some(Sthana::Kantha));
        assert_eq!(c.ghosa, Some(Ghosa::Aghosa));
        assert_eq!(c.prana, Some(Prana::Alpaprana));
    }

    #[test]
    fn classify_name_convenience() {
        assert_eq!(classify_name('a'), "svara");
        assert_eq!(classify_name('y'), "antaḥstha");
    }
}
