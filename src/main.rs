use crate::first_solutions::{day_1::trebuchet, day_2::cube_condundrum, day_3::gear_ratio_solution, day_4::scratchcard_solution};
use std::fs;

pub mod first_solutions;

fn main() {
    // day 1
    // let trebuchet_puzzle =
    //     fs::read_to_string("/home/clushh/rust/aoc_2023/src/puzzles/trebuchet_puzzle.txt").unwrap();
    // println!("{}", trebuchet(trebuchet_puzzle.as_str()));

    // day 2
    // let cube_puzzle = fs::read_to_string("/home/clushh/rust/aoc_2023/src/puzzles/cube_conundrum_puzzle.txt").unwrap();
    // print!("{}", cube_condundrum(cube_puzzle.as_str()));
    
    // day 3
    // let gear_puzzle = fs::read_to_string("/home/clushh/rust/aoc_2023/src/puzzles/gear_ratios_puzzle.txt").unwrap();
    // print!("{}", gear_ratio_solution(gear_puzzle.as_str()));

    // day 4
    let scratchcard_puzzle = fs::read_to_string("/home/clushh/rust/aoc_2023/src/puzzles/scratchcard_puzzle.txt").unwrap();
    print!("{}", scratchcard_solution(scratchcard_puzzle.as_str()));
}
