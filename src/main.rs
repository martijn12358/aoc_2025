pub mod aoc;
use crate::aoc::*;

fn main() {
    println!("answer: {}", aoc_1::solve_1_f( include_bytes!("aoc/input/full_input_aoc1.txt")));
    println!("answer: {}", aoc_1::solve_2(include_bytes!("aoc/input/full_input_aoc1.txt")));
    //println!("{:?}", include_bytes!("aoc/input/full_input_aoc1.txt"));
}

#[test]
fn test_aoc_1_1() {
    assert_eq!(aoc_1::solve_1( include_bytes!("aoc/input/test_input_aoc1.txt")), 3);
    assert_eq!(aoc_1::solve_1( include_bytes!("aoc/input/full_input_aoc1.txt")), 1195);
}

#[test]
fn test_aoc_1_2() {
    assert_eq!(aoc_1::solve_2f(include_bytes!("aoc/input/test_input_aoc1.txt")), 6);
    assert_eq!(aoc_1::solve_2f(include_bytes!("aoc/input/full_input_aoc1.txt")), 6770);
}
