use day_15::{puzzle1, puzzle2};
use std::fs;

/// --- Day 15: Beacon Exclusion Zone ---
/// 1. In the row where y=2000000, how many positions cannot contain a beacon?
/// 2. What is its tuning frequency?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file, 2_000_000));
    println!("Puzzle 2: {}", puzzle2(&file, (4_000_000, 4_000_000)));
}
