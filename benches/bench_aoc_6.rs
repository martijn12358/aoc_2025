use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use aoc2025::aoc::aoc_6;

fn bench_aoc_6_part1(c: &mut Criterion) {
    let input = black_box(include_str!("../src/aoc/input/full_input_aoc5.txt"));
    c.bench_function("bench_aoc6 part 1", |b| b.iter(|| aoc_6::solve_p1(input)));
}

fn bench_aoc_6_part2(c: &mut Criterion) {
    let input = black_box(include_str!("../src/aoc/input/full_input_aoc5.txt"));
    c.bench_function("bench_aoc6 part 2", |b| b.iter(|| aoc_6::solve_p2(input)));
}



criterion_group!(
    benches_p6,
    bench_aoc_6_part1,
    bench_aoc_6_part2,
);
criterion_main!(benches_p6);