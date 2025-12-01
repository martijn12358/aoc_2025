use aoc2025::aoc::aoc_1;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_aoc_1_part1(c: &mut Criterion) {
    let input = black_box(include_bytes!("../src/aoc/input/full_input_aoc1.txt"));
    c.bench_function("bench_aoc1 part", |b| b.iter(|| aoc_1::solve_1(input)));
}

fn bench_aoc_1_part2(c: &mut Criterion) {
    let input = black_box(include_bytes!("../src/aoc/input/full_input_aoc1.txt"));
    c.bench_function("bench_aoc1 part2", |b| b.iter(|| aoc_1::solve_2(input)));
}

fn bench_aoc_1_part2_f(c: &mut Criterion) {
    let input = black_box(include_bytes!("../src/aoc/input/full_input_aoc1.txt"));
    c.bench_function("bench_aoc2 part fast", |b| b.iter(|| aoc_1::solve_2f(input)));
}
criterion_group!(benches_p1, bench_aoc_1_part1, bench_aoc_1_part2, bench_aoc_1_part2_f);
criterion_main!(benches_p1);
