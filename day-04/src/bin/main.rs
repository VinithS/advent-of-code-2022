use day_04::{puzzle1, puzzle2};
use std::fs;

/// --- Day 4: Camp Cleanup ---
/// 1. In how many assignment pairs does one range fully contain the other?
/// 2. In how many assignment pairs do the ranges overlap?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
