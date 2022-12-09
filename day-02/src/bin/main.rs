use day_02::{puzzle1, puzzle2};
use std::fs;

/// --- Day 2: Rock Paper Scissors ---
/// 1. What would your total score be if everything goes exactly according to your strategy guide?
/// 2. What would your total score be if everything goes exactly according to the new strategy guide?
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Puzzle 1: {}", puzzle1(&file));
    println!("Puzzle 2: {}", puzzle2(&file));
}
