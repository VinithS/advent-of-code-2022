use day_11::{puzzle1, puzzle2};
use std::fs;

/// --- Day 11: Monkey in the Middle ---
/// 1. What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?
/// 2. What is the level of monkey business after 10_000 rounds?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
