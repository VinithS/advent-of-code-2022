use day_13::{puzzle1, puzzle2};
use std::fs;

/// --- Day 13: Distress Signal ---
/// 1. What is the sum of the indices of those pairs?
/// 2. What is the decoder key for the distress signal?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
