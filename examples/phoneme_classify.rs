//! Phase 1 demonstration: type-level phoneme categories and classification.
//!
//! Run with:
//! ```bash
//! cargo run --example phoneme_classify
//! ```

use tat_tvam_asi::mantra::{classify, Phoneme, Svara, Vyanjana, Antastha, Usman};

fn main() {
    println!("🪐 Phase 1 — Phoneme Categories (Pāṇinian-inspired)\n");

    // Typed phonemes — class is part of the type
    let a = Phoneme::<Svara>::new('a');
    let k = Phoneme::<Vyanjana>::new('k');
    let y = Phoneme::<Antastha>::new('y');
    let s = Phoneme::<Usman>::new('ś');

    println!("Typed phonemes:");
    println!("  '{}' → class '{}'", a.symbol(), a.class_name());
    println!("  '{}' → class '{}'", k.symbol(), k.class_name());
    println!("  '{}' → class '{}'", y.symbol(), y.class_name());
    println!("  '{}' → class '{}'", s.symbol(), s.class_name());

    println!("\nRuntime classification (closed match):");
    let samples = ['a', 'i', 'k', 't', 'y', 'r', 'ś', 'h', 'x'];
    for ch in samples {
        println!("  '{}' → {}", ch, classify(ch));
    }

    println!("\nZero-cost: the class marker adds no runtime size beyond the char.");
    println!("Tat Tvam Asi.");
}
