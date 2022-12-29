use parser::colony_parser;

mod monkey;
mod operation;
mod parser;

pub fn puzzle1(input: &str) -> u64 {
    let (_, mut col) = colony_parser(input).unwrap();

    for _ in 0..20 {
        for turn in 0..col.monkies.len() {
            col.play_turn(turn, None); // don't care
        }
    }
    return col
        .get_top_n_inspections(2)
        .iter()
        .fold(1, |acc: u64, i: &u64| acc * i);
}

pub fn puzzle2(input: &str) -> u64 {
    let (_, mut col) = colony_parser(input).unwrap();
    let m = col.get_meditation_number();

    for _ in 0..10_000 {
        for turn in 0..col.monkies.len() {
            col.play_turn(turn, Some(m)); // we care X_X
        }
    }

    return col
        .get_top_n_inspections(2)
        .iter()
        .fold(1, |acc: u64, i: &u64| acc * i);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, 2713310158);
    }
}
