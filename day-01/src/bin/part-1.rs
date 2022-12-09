use day_01::part1;
use std::fs;

// Calorie Counting
fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", part1(&file));
}
