

pub mod aoc;
use crate::aoc::*;

fn main() {
    let input = include_str!("aoc/input/full_input_aoc2.txt");
    let answer = aoc_2::solve_p1(input);
    println!("{}", answer);

    let input = include_bytes!("aoc/input/test_input_aoc2.txt");
    //let answer = aoc_2::solve_p1_f(input);
    println!("{:?}", input);
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

#[test]
fn test_aoc_2_part1() {
    assert_eq!(aoc_2::solve_p1(include_str!("aoc/input/test_input_aoc2.txt")), 1227775554);
}
#[test]
fn test_aoc_2_part2() {
    assert_eq!(aoc_2::solve_p2(include_str!("aoc/input/test_input_aoc2.txt")), 4174379265);
}