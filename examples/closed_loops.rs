//! Integrated demonstration of the three closed loops.
//!
//! Run with:
//! ```bash
//! cargo run --example closed_loops
//! ```

use tat_tvam_asi::{
    essence::{Essence, Form, Moksha, Om},
    law::Resource,
    sunyata::SacredIterator,
    vision::State,
};

/// A small domain type that must be handled exhaustively.
#[derive(Debug, Clone, Copy)]
enum Action {
    Observe,
    Decide,
    Execute,
    Liberate,
}

fn handle_action(action: Action) -> &'static str {
    // Dayan’s eye: every possibility is named. No `_`.
    match action {
        Action::Observe => "world observed",
        Action::Decide => "intent formed",
        Action::Execute => "constrained action taken",
        Action::Liberate => "cycle released",
    }
}

fn main() {
    println!("🪐 Closed Loops — Integrated Demonstration\n");

    // ── Loop 1: Karma & RAII ──────────────────────────────────────
    println!("[Loop 1] Karma & RAII");
    {
        let res = Resource::new(108, b"tat tvam asi".to_vec());
        println!("  born  → id={}", res.id());
        let payload = res.into_data();
        println!("  moved → {:?}", String::from_utf8_lossy(&payload));
        // res is gone. Drop will run at scope end.
    }
    println!("  died  → no residual karma\n");

    // ── Loop 2: Essence & Exhaustive Vision ───────────────────────
    println!("[Loop 2] Essence (Nāmarūpa) + Exhaustive Match");

    let om = Form::new(Om);
    let moksha = Form::new(Moksha);
    println!("  {} → {}", om.name(), om.manifest());
    println!("  {} → {}", moksha.name(), moksha.manifest());

    let actions = [
        Action::Observe,
        Action::Decide,
        Action::Execute,
        Action::Liberate,
    ];
    for a in actions {
        println!("  {:?} → {}", a, handle_action(a));
    }

    let mut state = State::Unborn;
    print!("  state path: {:?}", state);
    while !state.is_final() {
        state = state.transition();
        print!(" → {:?}", state);
    }
    println!("\n");

    // ── Loop 3: Śūnyatā (Zero-Cost) ───────────────────────────────
    println!("[Loop 3] Śūnyatā — Zero-Cost Sacred Map");
    let data = [1u32, 2, 3, 4, 5, 6, 7, 8];
    let result: Vec<u32> = data
        .iter()
        .sacred_map(|x| x * x)      // square
        .sacred_map(|x| x + 1)      // +1
        .filter(|&x| x % 2 == 1)    // keep odd
        .collect();

    println!("  input  : {:?}", data);
    println!("  sacred : {:?}", result);
    println!("  (all adapters erased at compile time)\n");

    println!("Three closed loops demonstrated in one binary.");
    println!("Tat Tvam Asi.");
}
