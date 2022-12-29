use std::collections::HashMap;

use graph::{create_elevation_graph_bfs, create_elevation_graph_dfs, create_grid_and_config};

mod graph;

pub fn puzzle1(input: &str) -> usize {
    let (grid, config) = create_grid_and_config(input, false);

    let graph: HashMap<(usize, usize), ((usize, usize), usize)> =
        create_elevation_graph_bfs(grid, config.start);

    match graph.get(&config.end) {
        Some(ans) => ans.1,
        None => 0,
    }
}

pub fn puzzle2(input: &str) -> u64 {
    let (grid, config) = create_grid_and_config(input, true);

    let graph: HashMap<(usize, usize), usize> = create_elevation_graph_dfs(grid, config.start);

    match graph.get(&config.end) {
        Some(ans) => *ans as u64,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, 29);
    }

    const INPUT_LARGE: &str = include_str!("../test.txt");

    #[test]
    fn test_puzzle1_large() {
        let result = puzzle1(INPUT_LARGE);
        assert_eq!(result, 322);
    }
}
