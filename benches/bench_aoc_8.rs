use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use aoc2025::aoc::aoc_8;

fn bench_aoc_8_part1(c: &mut Criterion) {
    let input = black_box(include_str!("../src/aoc/input/full_input_aoc8.txt"));
    c.bench_function("bench_aoc8 part 1", |b| b.iter(|| aoc_8::solve_p1(input)));
}


criterion_group!(
    benches_p8,
    bench_aoc_8_part1,
);
criterion_main!(benches_p8);