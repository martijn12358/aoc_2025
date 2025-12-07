#![feature(portable_simd)] // Enable the nightly feature
pub mod aoc;
use crate::aoc::*;

fn main() {
    /*
    let input = include_bytes!("aoc/input/full_input_aoc3.txt");
    let answer = aoc_3::solve_p1(input);
    println!("{}", answer);
    let input2 = include_bytes!("aoc/input/full_input_aoc3.txt");
    let answer = aoc_3::solve_p1_fast(input2);
    println!("{}", answer);
     */
    /*
    let input = include_bytes!("aoc/input/full_input_aoc4.txt");
    let answer = aoc_4::solve_p1_f(input);
    println!("{}", answer);

    let input = include_bytes!("aoc/input/full_input_aoc4.txt");
    let answer = aoc_4::solve_p1(input);
    println!("{}", answer);
     */
    /*
    let input = include_str!("aoc/input/full_input_aoc5.txt");
    let answer = aoc_5::solve_p1(input);
    println!("{}", answer);

    let input = include_str!("aoc/input/test_input_aoc5.txt");
    let answer = aoc_5::solve_p2(input);
    println!("{}", answer);

     */
    /*
    let input = include_str!("aoc/input/full_input_aoc6.txt");
    let answer = aoc_6::solve_p1(input);
    println!("answer: {}", answer);

    let input = include_str!("aoc/input/full_input_aoc6.txt");
    let answer = aoc_6::solve_p2(input);
    println!("answer: {}", answer);

     */

    let input = include_str!("aoc/input/full_input_aoc7.txt");
    let answer = aoc_7::solve_p1(input);
    println!("answer: {}", answer);
    let input = include_str!("aoc/input/test_input_aoc7.txt");
    let answer = aoc_7::solve_p2(input);
    println!("answer: {}", answer);

}

#[test]
fn test_aoc_1_1() {
    assert_eq!(
        aoc_1::solve_1(include_bytes!("aoc/input/test_input_aoc1.txt")),
        3
    );
    assert_eq!(
        aoc_1::solve_1(include_bytes!("aoc/input/full_input_aoc1.txt")),
        1195
    );
}

#[test]
fn test_aoc_1_2() {
    assert_eq!(
        aoc_1::solve_2f(include_bytes!("aoc/input/test_input_aoc1.txt")),
        6
    );
    assert_eq!(
        aoc_1::solve_2f(include_bytes!("aoc/input/full_input_aoc1.txt")),
        6770
    );
}

#[test]
fn test_aoc_2_part1() {
    assert_eq!(
        aoc_2::solve_p1(include_str!("aoc/input/test_input_aoc2.txt")),
        1227775554
    );
}
#[test]
fn test_aoc_2_part2() {
    assert_eq!(
        aoc_2::solve_p2(include_str!("aoc/input/test_input_aoc2.txt")),
        4174379265
    );
}

#[test]
fn test_aoc_3_part1() {
    assert_eq!(
        aoc_3::solve_p1(include_bytes!("aoc/input/full_input_aoc3.txt")),
        17359
    );
}
#[test]
fn test_aoc_3_part2() {
    assert_eq!(
        aoc_3::solve_p2_fast(include_bytes!("aoc/input/full_input_aoc3.txt")),
        172787336861064
    );
}
