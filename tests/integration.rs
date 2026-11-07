//! Integration tests — full-path verification of the closed loops.

use tat_tvam_asi::{
    classify, classify_name,
    essence::{Essence, Form, Om},
    law::Resource,
    process::{phase_to_state, Phase, SacredProcess},
    run_typed_pipeline_owned,
    sunyata::SacredIterator,
    vision::State,
    Ghosa, Prana, Sthana,
};

#[test]
fn law_ownership_roundtrip() {
    let r = Resource::new(7, b"karma".to_vec());
    assert_eq!(r.id(), 7);
    let data = r.into_data();
    assert_eq!(&data, b"karma");
}

#[test]
fn vision_reaches_liberation() {
    let mut s = State::Unborn;
    for _ in 0..4 {
        if s.is_final() {
            break;
        }
        s = s.transition();
    }
    assert!(s.is_final());
    assert_eq!(s, State::Liberated);
}

#[test]
fn essence_form_manifests() {
    let f = Form::new(Om);
    assert_eq!(f.name(), "ॐ");
    assert!(!f.manifest().is_empty());
}

#[test]
fn sunyata_sacred_map_pipeline() {
    let xs = [1u32, 2, 3];
    let out: Vec<_> = xs
        .iter()
        .sacred_map(|x| x + 1)
        .sacred_map(|x| x * 2)
        .collect();
    assert_eq!(out, vec![4, 6, 8]);
}

#[test]
fn process_completes_with_single_output() {
    let mut p = SacredProcess::new(1, 5u32);
    let mut calls = 0;
    while !p.is_complete() {
        p.step(|input, phase| {
            calls += 1;
            input + phase as u32
        });
    }
    assert_eq!(calls, 1);
    assert!(p.output().is_some());
    assert!(p.into_output().is_some());
}

#[test]
fn phase_to_state_is_total() {
    for phase in [
        Phase::Observe,
        Phase::Intend,
        Phase::Predict,
        Phase::Execute,
        Phase::Evaluate,
        Phase::Update,
    ] {
        let _ = phase_to_state(phase);
    }
}

#[test]
fn mantra_classify_core_sounds() {
    assert_eq!(classify_name('a'), "svara");
    assert_eq!(classify_name('y'), "antaḥstha");
    assert_eq!(classify_name('ś'), "ūṣman");

    let k = classify('k');
    assert_eq!(k.class, "vyañjana");
    assert_eq!(k.sthana, Some(Sthana::Kantha));
    assert_eq!(k.ghosa, Some(Ghosa::Aghosa));
    assert_eq!(k.prana, Some(Prana::Alpaprana));
}

#[test]
fn typed_pipeline_owned_reaches_update() {
    let (done, out) = run_typed_pipeline_owned(9, 100u32, |x| x / 2);
    assert_eq!(out, 50);
    assert_eq!(done.phase_name(), "Update");
    assert_eq!(done.cosmic_state(), State::Liberated);
    assert_eq!(done.into_input(), 100);
}
