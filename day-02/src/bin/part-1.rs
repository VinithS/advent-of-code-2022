use day_02::{part1, part2};
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", part1(&file));
    println!("{}", part2(&file));
}
