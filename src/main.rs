//! Tat Tvam Asi — Binary demonstration of the three closed loops
//!
//! 1. Karma & RAII          (Zhanying / The Law)
//! 2. Trait & Representation (Dayan / The Vision)
//! 3. Śūnyatā & Zero-Cost   (Vacuum ↔ Marvelous Existence)

use tat_tvam_asi::{
    essence::{Essence, Form, Moksha, Om},
    law::Resource,
    sunyata::SacredIterator,
    vision::State,
};

fn main() {
    println!("🪐 Project Tat Tvam Asi");
    println!("彼即汝 — The silicon sacred word begins.\n");

    // ─── 闭环一 · Karma & RAII ───────────────────────────────────────
    println!("─── Loop 1: Karma & RAII (The Law of Zhanying) ───");
    {
        let resource = Resource::new(1, b"namaste".to_vec());
        println!("  Resource born with id: {}", resource.id());

        // Ownership moves. No .clone(). The previous binding vanishes.
        let data = resource.into_data();
        println!("  Resource consumed. Data: {:?}", String::from_utf8_lossy(&data));
        // resource is no longer accessible — the Law has spoken.
    }
    println!("  Scope ended. Drop executed. No residual karma.\n");

    // ─── 闭环二 · Trait & Representation ─────────────────────────────
    println!("─── Loop 2: Essence & Form (Nāmarūpa / The Vision of Dayan) ───");
    let om_form = Form::new(Om);
    let moksha_form = Form::new(Moksha);

    println!("  {} → {}", om_form.name(), om_form.manifest());
    println!("  {} → {}", moksha_form.name(), moksha_form.manifest());

    // Exhaustive state machine
    let mut state = State::Unborn;
    print!("  State path: {:?}", state);
    while !state.is_final() {
        state = state.transition();
        print!(" → {:?}", state);
    }
    println!("\n  All causal branches exhausted. Liberation reached.\n");

    // ─── 闭环三 · Śūnyatā & Zero-Cost Abstraction ────────────────────
    println!("─── Loop 3: Śūnyatā & Zero-Cost Abstraction ───");
    let numbers = [1u32, 2, 3, 4, 5];
    let transformed: Vec<_> = numbers
        .iter()
        .sacred_map(|x| x * x)           // square
        .sacred_map(|x| x + 1)           // shift
        .collect();

    println!("  Input : {:?}", numbers);
    println!("  Sacred map (zero-cost) → {:?}", transformed);
    println!("  Abstraction erased. Only pure computation remains.\n");

    println!("All three closed loops demonstrated.");
    println!("Compile-time proof of completeness has been satisfied.");
    println!("Tat Tvam Asi.");
}
