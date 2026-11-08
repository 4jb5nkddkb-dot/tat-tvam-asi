//! Criterion bench: SacredMap should track std map closely (zero-cost intent).
//!
//! ```bash
//! cargo bench --bench sacred_map
//! ```

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tat_tvam_asi::SacredIterator;

fn pipeline_std(data: &[u32]) -> u64 {
    data.iter()
        .map(|x| x.wrapping_mul(3))
        .map(|x| x.wrapping_add(1))
        .map(|x| x.wrapping_mul(x))
        .fold(0u64, |acc, x| acc.wrapping_add(x as u64))
}

fn pipeline_sacred(data: &[u32]) -> u64 {
    data.iter()
        .sacred_map(|x| x.wrapping_mul(3))
        .sacred_map(|x| x.wrapping_add(1))
        .sacred_map(|x| x.wrapping_mul(x))
        .fold(0u64, |acc, x| acc.wrapping_add(x as u64))
}

fn bench_pipelines(c: &mut Criterion) {
    let data: Vec<u32> = (0..10_000).collect();

    c.bench_function("std_map_pipeline", |b| {
        b.iter(|| pipeline_std(black_box(&data)))
    });

    c.bench_function("sacred_map_pipeline", |b| {
        b.iter(|| pipeline_sacred(black_box(&data)))
    });
}

criterion_group!(benches, bench_pipelines);
criterion_main!(benches);
