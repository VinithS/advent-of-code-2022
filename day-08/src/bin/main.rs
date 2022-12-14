use day_08::{puzzle1, puzzle2};
use std::fs;

/// --- Day 8: Treetop Tree House ---
/// 1. How many trees are visible from outside the grid?
/// 2. What is the highest scenic score possible for any tree?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
