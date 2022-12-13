use day_07::{puzzle1, puzzle2};
use std::fs;

/// --- Day 7: No Space Left On Device ---
/// 1. What is the sum of the total sizes of those directories?
/// 2.
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
