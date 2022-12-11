use day_06::{puzzle1, puzzle2};
use std::fs;

/// --- Day 6: Tuning Trouble ---
/// 1. How many characters need to be processed before the first start-of-packet marker is detected?
/// 2. How many characters need to be processed before the first start-of-message marker is detected?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
