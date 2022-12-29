use day_12::{puzzle1, puzzle2};
use std::fs;

/// --- Day 12: Hill Climbing Algorithm ---
/// 1. What is the fewest steps required to move from your current position to the location that should get the best signal?
/// 2. What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
