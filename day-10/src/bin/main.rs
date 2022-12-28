use day_10::{puzzle1, puzzle2};
use std::fs;

/// --- Day 10: Cathode-Ray Tube ---
/// 1. What is the sum of these six signal strengths?
/// 2. What eight capital letters appear on your CRT?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: \n{}", puzzle2(&file)); // BRJLFULP
}
