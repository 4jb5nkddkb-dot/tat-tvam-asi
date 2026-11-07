//! Phase 1 demonstration: expanded phoneme classification.
//!
//! Run with:
//! ```bash
//! cargo run --example phoneme_classify
//! ```

use tat_tvam_asi::mantra::{classify, Antastha, Phoneme, Svara, Usman, Vyanjana};

fn main() {
    println!("🪐 Phase 1 — Expanded Phoneme Categories\n");

    let a = Phoneme::<Svara>::new('a');
    let k = Phoneme::<Vyanjana>::new('k');
    let y = Phoneme::<Antastha>::new('y');
    let s = Phoneme::<Usman>::new('ś');

    println!("Typed phonemes:");
    println!("  '{}' → {}", a.symbol(), a.class_name());
    println!("  '{}' → {}", k.symbol(), k.class_name());
    println!("  '{}' → {}", y.symbol(), y.class_name());
    println!("  '{}' → {}", s.symbol(), s.class_name());

    println!("\nRich classification:");
    for ch in ['a', 'k', 'g', 'y', 'ś', 'x'] {
        let c = classify(ch);
        println!(
            "  '{}' → class={}, sthana={:?}, prayatna={:?}, ghosa={:?}, prana={:?}",
            ch, c.class, c.sthana, c.prayatna, c.ghosa, c.prana
        );
    }

    println!("\nTat Tvam Asi.");
}
