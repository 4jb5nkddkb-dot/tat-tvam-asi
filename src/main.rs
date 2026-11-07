//! Tat Tvam Asi — Binary demonstration of the three closed loops
//! and the integrated SacredProcess.

use tat_tvam_asi::{
    essence::{Essence, Form, Moksha, Om},
    law::Resource,
    process::{phase_to_state, SacredProcess},
    sunyata::SacredIterator,
    vision::State,
};

fn main() {
    println!("🪐 Project Tat Tvam Asi");
    println!("彼即汝 — The silicon sacred word begins.\n");

    // ─── Loop 1: Karma & RAII ─────────────────────────────────────
    println!("─── Loop 1: Karma & RAII (The Law of Zhanying) ───");
    {
        let resource = Resource::new(1, b"namaste".to_vec());
        println!("  Resource born with id: {}", resource.id());
        let data = resource.into_data();
        println!("  Resource consumed. Data: {:?}", String::from_utf8_lossy(&data));
    }
    println!("  Scope ended. Drop executed. No residual karma.\n");

    // ─── Loop 2: Trait & Representation + Exhaustive Vision ───────
    println!("─── Loop 2: Essence & Exhaustive Vision (Dayan) ───");
    let om = Form::new(Om);
    let moksha = Form::new(Moksha);
    println!("  {} → {}", om.name(), om.manifest());
    println!("  {} → {}", moksha.name(), moksha.manifest());

    let mut state = State::Unborn;
    print!("  Cosmic state path: {:?}", state);
    while !state.is_final() {
        state = state.transition();
        print!(" → {:?}", state);
    }
    println!("\n");

    // ─── Loop 3: Śūnyatā (Zero-Cost) ──────────────────────────────
    println!("─── Loop 3: Śūnyatā & Zero-Cost Abstraction ───");
    let numbers = [1u32, 2, 3, 4, 5];
    let transformed: Vec<_> = numbers
        .iter()
        .sacred_map(|x| x * x)
        .sacred_map(|x| x + 1)
        .collect();
    println!("  Input : {:?}", numbers);
    println!("  Sacred map → {:?}", transformed);
    println!("  Abstraction erased.\n");

    // ─── Integrated: SacredProcess ────────────────────────────────
    println!("─── Integrated: SacredProcess ───");
    let mut proc = SacredProcess::new(108, 42u32);
    println!("  Process {} started at phase {:?}", proc.id(), proc.phase());

    while !proc.is_complete() {
        let current = proc.phase();
        proc.step(|input, phase| {
            input.wrapping_mul(2).wrapping_add(phase as u32)
        });
        println!(
            "  phase {:?} → cosmic {:?}",
            current,
            phase_to_state(current)
        );
    }

    if let Some(result) = proc.into_output() {
        println!("  Process completed. Output: {}", result);
    }

    println!("\nAll closed loops and SacredProcess demonstrated.");
    println!("Tat Tvam Asi.");
}
