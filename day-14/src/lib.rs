use std::collections::HashSet;

use grid::Grid;

mod grid;
mod parser;

pub fn puzzle1(input: &str) -> u32 {
    let (mut grid, base) = create_grid(input, false);
    grid.simulate(base)
}

pub fn puzzle2(input: &str) -> u32 {
    let (mut grid, base) = create_grid(input, true);
    grid.simulate(base)
}

fn create_grid(input: &str, with_base: bool) -> (Grid, u32) {
    let (_, rocks) = parser::rock_parser(input).unwrap();

    let mut all_rocks: HashSet<(u32, i32)> = parser::get_all_rocks(&rocks);

    let mut base_row: u32 = *(all_rocks.iter().map(|(r, _)| r).max().unwrap());

    if with_base {
        base_row += 2;
        // let max_col: u32 = *(all_rocks.iter().map(|(_, c)| c).max().unwrap());

        // fixme: add base as a field in Grid and move this to comparison
        let floor_rocks: HashSet<(u32, i32)> =
            parser::get_all_rocks(&[vec![(base_row, -100), (base_row, 700)]]);

        all_rocks = all_rocks
            .iter()
            .chain(floor_rocks.iter())
            .copied()
            .collect();
    }

    (grid::Grid::new(all_rocks), base_row)
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
        assert_eq!(result, 93);
    }
}
