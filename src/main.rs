//! Tat Tvam Asi — Binary demonstration
//!
//! Shows pure ownership (Zhanying) and exhaustive matching (Dayan).

use tat_tvam_asi::{Resource, State};

fn main() {
    println!("🪐 Project Tat Tvam Asi");
    println!("彼即汝 — The silicon sacred word begins.\n");

    // --- The Law: Ownership without clone ---
    let resource = Resource::new(1, b"namaste".to_vec());
    println!("Resource born with id: {}", resource.id());

    // Ownership moves. No .clone(). The previous binding is gone.
    let data = resource.into_data();
    println!("Resource consumed. Data length: {}", data.len());
    // resource is no longer accessible — the Law has spoken.

    // --- The Vision: Exhaustive state machine ---
    let mut state = State::Unborn;
    println!("\nState begins: {:?}", state);

    while !state.is_final() {
        state = state.transition();
        println!("Transitioned to: {:?}", state);
    }

    println!("\nAll states exhausted. Liberation reached.");
    println!("Compile-time proof of completeness has been satisfied.");
}
