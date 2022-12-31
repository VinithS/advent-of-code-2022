use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

pub fn get_all_rocks(rocks: &[Vec<(u32, u32)>]) -> HashSet<(u32, u32)> {
    rocks
        .iter()
        .flat_map(|verticies: &Vec<(u32, u32)>| -> HashSet<(u32, u32)> {
            verticies
                .windows(2)
                .flat_map(|pair| generate_path_between(&pair[0], &pair[1]))
                .collect()
        })
        .collect()
}

pub fn rock_parser(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            separated_pair(u32, tag(","), u32).map(|(c, r)| (r, c)),
        ),
    )(input)
}

fn generate_path_between(p1: &(u32, u32), p2: &(u32, u32)) -> HashSet<(u32, u32)> {
    match get_dir(p1, p2) {
        Dir::D => (p2.0..=p1.0).map(|row| (row, p1.1)).collect(),
        Dir::U => (p1.0..=p2.0).map(|row| (row, p1.1)).collect(),
        Dir::R => (p2.1..=p1.1).map(|col| (p1.0, col)).collect(),
        Dir::L => (p1.1..=p2.1).map(|col| (p1.0, col)).collect(),
    }
}

fn get_dir(p1: &(u32, u32), p2: &(u32, u32)) -> Dir {
    // if rows are the same
    if p1.0 == p2.0 {
        if p1.1 > p2.1 {
            Dir::R
        } else {
            Dir::L
        }
    } else {
        // if p1.1 == p2.1 // columns are the same
        if p1.0 > p2.0 {
            Dir::D
        } else {
            Dir::U
        }
    }
}

enum Dir {
    U,
    D,
    L,
    R,
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_basic_parsing() {
        assert_eq!(
            rock_parser(INPUT),
            Ok((
                "",
                vec![
                    vec![(4, 498), (6, 498), (6, 496)],
                    vec![(4, 503), (4, 502), (9, 502), (9, 494)]
                ]
            ))
        );
    }
}
