use day_03::{puzzle1, puzzle2};
use std::fs;

/// --- Day 3: Rucksack Reorganization ---
/// 1. What is the sum of the priorities of common items within the two sets of rucksack?
/// 2. What is the sum of the priorities of common items between three sacks?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
