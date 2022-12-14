use day_08::{puzzle1, puzzle2};
use std::fs;

/// --- Day 8: Treetop Tree House ---
/// 1. How many characters need to be processed before the first start-of-packet marker is detected?
/// 2.
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
