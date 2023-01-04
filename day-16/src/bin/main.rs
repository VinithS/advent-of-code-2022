use day_16::{puzzle1, puzzle2};
use std::fs;

/// --- Day 16: Proboscidea Volcanium ---
/// 1. What is the most pressure you can release?
/// 2.
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
