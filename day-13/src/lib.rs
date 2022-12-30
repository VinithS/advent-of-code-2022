use std::cmp::Ordering;

use parser::ppp;

mod packet;
mod parser;

pub fn puzzle1(input: &str) -> usize {
    let mut counter = 0;

    for (idx, pp) in input.split("\n\n").enumerate() {
        let (_, pair) = ppp(pp).unwrap();

        let p1 = pair.0;
        let p2 = pair.1;

        // if p1.lt(&p2) {
        //     counter = counter + idx + 1;
        // }

        match p1.partial_cmp(&p2) {
            Some(ord) => match ord {
                Ordering::Less => {
                    println!(
                        "Left is smaller -> {} + {} = {}",
                        counter,
                        idx + 1,
                        counter + idx + 1
                    );
                    counter = counter + idx + 1;
                }
                Ordering::Equal => {
                    println!("They are equal.");
                }
                Ordering::Greater => {
                    println!("Left is greater.");
                }
            },
            None => {
                // failed to compare. check parser.rs
                todo!()
            }
        }
    }
    return counter;
}

pub fn puzzle2(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, 29);
    }
}
