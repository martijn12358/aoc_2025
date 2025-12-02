use aoc2025::aoc::aoc_2;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_aoc_2_part1(c: &mut Criterion) {
    let input = black_box(include_str!("../src/aoc/input/full_input_aoc2.txt"));
    c.bench_function("bench_aoc2 part 1", |b| b.iter(|| aoc_2::solve_p1(input)));
}

fn bench_aoc_2_part2(c: &mut Criterion) {
    let input = black_box(include_str!("../src/aoc/input/full_input_aoc2.txt"));
    c.bench_function("bench_aoc2 part 1", |b| b.iter(|| aoc_2::solve_p2(input)));
}

criterion_group!(benches_p2, bench_aoc_2_part1, bench_aoc_2_part2);
criterion_main!(benches_p2);