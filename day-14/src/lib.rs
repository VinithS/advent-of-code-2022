use std::collections::HashSet;

mod grid;
mod parser;

pub fn puzzle1(input: &str) -> u32 {
    let (_, rocks) = parser::rock_parser(input).unwrap();

    let all_rocks: HashSet<(u32, u32)> = parser::get_all_rocks(&rocks);

    // seems to be 9 like the example
    let base_level: u32 = *(all_rocks.iter().map(|(r, _)| r).max().unwrap());

    let mut grid = grid::Grid::new(all_rocks);

    grid.simulate(base_level)
}

pub fn puzzle2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, 0);
    }
}
