use crate::first_solutions::{day_1::trebuchet, day_2::cube_condundrum};
use std::fs;

pub mod first_solutions;

fn main() {
    // let trebuchet_puzzle =
    //     fs::read_to_string("/home/clushh/rust/aoc_2023/src/puzzles/trebuchet_puzzle.txt").unwrap();
    // println!("{}", trebuchet(trebuchet_puzzle.as_str()));

    let cube_puzzle = fs::read_to_string("/home/clushh/rust/aoc_2023/src/puzzles/cube_conundrum_puzzle.txt").unwrap();
    print!("{}", cube_condundrum(cube_puzzle.as_str()));
}
