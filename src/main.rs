// use std::env;
use std::fs;

mod dayone;
mod daytwo;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let filepath = "./lib/day-2.txt";

    let contents = fs::read_to_string(filepath).expect("Input file missing.");

    // dayone::puzzle(&contents);
    daytwo::puzzle(&contents);
}
