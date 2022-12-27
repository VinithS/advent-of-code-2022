use day_09::{puzzle1, puzzle2};
use std::fs;

/// --- Day 9: Rope Bridge ---
/// 1. How many positions does the tail of the rope visit at least once?
/// 2. What if there were 10 knots?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
