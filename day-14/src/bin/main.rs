use day_14::{puzzle1, puzzle2};
use std::fs;

/// --- Day 14: Regolith Reservoir ---
/// 1. How many units of sand come to rest before sand starts flowing into the abyss below?
/// 2. How many units of sand come to rest?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
