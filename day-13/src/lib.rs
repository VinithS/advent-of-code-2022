use std::cmp::Ordering::*;

use packet::Packet;
use parser::ppp;

mod packet;
mod parser;

pub fn puzzle1(input: &str) -> usize {
    let (_, packets) = ppp(input).unwrap();

    packets
        .iter()
        .enumerate()
        .filter_map(|(i, (left_p, right_p))| -> Option<usize> {
            match left_p.cmp(right_p) {
                Less => Some(i + 1),
                Equal | Greater => None,
            }
        })
        .sum::<usize>()
}

pub fn puzzle2(input: &str) -> usize {
    let (_, packets) = ppp(input).unwrap();

    let dp2 = Packet::decode_packet(2);
    let dp6 = Packet::decode_packet(6);

    let mut sorted: Vec<&Packet> = packets
        .iter()
        .flat_map(|(p1, p2)| [p1, p2])
        .chain([&dp2, &dp6])
        .collect();
    sorted.sort();

    let dp2_loc: usize = sorted.iter().position(|&r| r == &dp2).unwrap() + 1;
    let dp6_loc: usize = sorted.iter().position(|&r| r == &dp6).unwrap() + 1;

    dp2_loc * dp6_loc
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
        assert_eq!(result, 140);
    }
}
