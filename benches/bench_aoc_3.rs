use aoc2025::aoc::aoc_3;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_aoc_3_part1(c: &mut Criterion) {
    let input = black_box(include_bytes!("../src/aoc/input/full_input_aoc3.txt"));
    c.bench_function("bench_aoc3 part 1", |b| b.iter(|| aoc_3::solve_p1(input)));
}
fn bench_aoc_3_part1_fast(c: &mut Criterion) {
    let input = black_box(include_bytes!("../src/aoc/input/full_input_aoc3.txt"));
    c.bench_function("bench_aoc3 part 1 fast", |b| {
        b.iter(|| aoc_3::solve_p1_fast(input))
    });
}

fn bench_aoc_3_part2(c: &mut Criterion) {
    let input = black_box(include_bytes!("../src/aoc/input/full_input_aoc3.txt"));
    c.bench_function("bench_aoc3 part 2", |b| b.iter(|| aoc_3::solve_p2(input)));
}

fn bench_aoc_3_part2_fast(c: &mut Criterion) {
    let input = black_box(include_bytes!("../src/aoc/input/full_input_aoc3.txt"));
    c.bench_function("bench_aoc3 part 2 fast", |b| {
        b.iter(|| aoc_3::solve_p2_fast(input))
    });
}

criterion_group!(
    benches_p3,
    bench_aoc_3_part1,
    bench_aoc_3_part1_fast,
    bench_aoc_3_part2,
    bench_aoc_3_part2_fast
);
criterion_main!(benches_p3);
