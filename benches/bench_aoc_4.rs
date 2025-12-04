use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use aoc2025::aoc::aoc_4;

fn bench_aoc_4_part1(c: &mut Criterion) {
    let input = black_box(include_bytes!("../src/aoc/input/full_input_aoc4.txt"));
    c.bench_function("bench_aoc4 part 1", |b| b.iter(|| aoc_4::solve_p1(input)));
}
fn bench_aoc_4_part1_f(c: &mut Criterion) {
    let input = black_box(include_bytes!("../src/aoc/input/full_input_aoc4.txt"));
    c.bench_function("bench_aoc4 part 1 fast", |b| b.iter(|| aoc_4::solve_p1_f(input)));
}
fn bench_aoc_4_part2(c: &mut Criterion) {
    let input = black_box(include_str!("../src/aoc/input/full_input_aoc4.txt"));
    c.bench_function("bench_aoc4 part 2", |b| b.iter(|| aoc_4::solve_p2(input)));
}
fn bench_aoc_4_part2_f(c: &mut Criterion) {
    let input = black_box(include_bytes!("../src/aoc/input/full_input_aoc4.txt"));
    c.bench_function("bench_aoc4 part 2", |b| b.iter(|| aoc_4::solve_p2_f(input)));
}

criterion_group!(
    benches_p4,
    bench_aoc_4_part1,
    bench_aoc_4_part1_f,
    bench_aoc_4_part2,
    bench_aoc_4_part2_f
);
criterion_main!(benches_p4);