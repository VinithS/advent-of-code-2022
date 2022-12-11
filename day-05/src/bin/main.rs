use day_05::{puzzle1, puzzle2};
use std::fs;

/// --- Day 5: Supply Stacks ---
/// 1. Which crate will end up on top of each stack?
/// 2. What if the crane could pick up multiple crates?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
