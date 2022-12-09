use day_01::{puzzle1, puzzle2};
use std::fs;

/// --- Day 1: Calorie Counting ---
/// 1. How many total Calories is that Elf carrying?
/// 2. How many Calories are those Elves carrying in total?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
