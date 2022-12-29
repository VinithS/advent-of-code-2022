use std::collections::{HashMap, HashSet};

pub fn create_elevation_graph(
    grid: Vec<Vec<char>>,
    sp: (usize, usize),
) -> HashMap<(usize, usize), ((usize, usize), usize)> {
    let grid_row = grid.len();
    let grid_col = grid[0].len();

    let mut graph: HashMap<(usize, usize), ((usize, usize), usize)> = HashMap::new();
    let mut queue = vec![sp]; // starting point for bfs
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    visited.insert(sp);

    while !queue.is_empty() {
        let current_point: (usize, usize) = queue.remove(0);

        let current_elevation: i32 = grid[current_point.0][current_point.1] as i32 - 'a' as i32;

        for neighbor in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let loc: (usize, usize) = (
                (current_point.0 as i32 + neighbor.0) as usize,
                (current_point.1 as i32 + neighbor.1) as usize,
            );

            // no need to < 0 check due to type limits :)
            if (loc.0 < grid_row) && (loc.1 < grid_col) {
                println!(
                    "  {}({}) to {}({})= {}",
                    grid[loc.0][loc.1],
                    grid[loc.0][loc.1] as i32 - 'a' as i32,
                    grid[current_point.0][current_point.1],
                    current_elevation,
                    (grid[loc.0][loc.1] as i32 - 'a' as i32 - current_elevation).abs()
                );

                if (grid[loc.0][loc.1] as i32 - 'a' as i32 - current_elevation).abs() <= 1 {
                    if !visited.contains(&loc) {
                        let len_to_curr = match graph.get(&current_point) {
                            Some((_, length)) => *length + 1,
                            None => 1,
                        };

                        graph.insert(loc, (current_point, len_to_curr));
                        visited.insert(loc);
                        queue.push(loc);

                        println!(
                            "    visited {:?} -> {}",
                            &loc, grid[current_point.0][current_point.1]
                        );
                    }
                }
            }
        }
    }

    graph
}

pub fn create_grid_and_config(input: &str) -> (Vec<Vec<char>>, Config) {
    let mut ep = (0, 0);
    let mut sp = (0, 0);

    let max_elev = 123 as char; // one more than 'z'
    let min_elev = 96 as char; // one less than 'a

    let g = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    'E' => max_elev,
                    'S' => min_elev,
                    _ => c,
                })
                .collect()
        })
        .collect::<Vec<Vec<char>>>();

    for r in 0..g.len() {
        for c in 0..g[0].len() {
            if g[r][c] == min_elev {
                sp = (r, c);
            }
            if g[r][c] == max_elev {
                ep = (r, c);
            }
        }
    }

    return (g, Config { start: sp, end: ep });
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    // Sab.. -> aab..
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_grid_creation() {
        assert_eq!(
            create_grid_and_config(INPUT),
            (
                vec![
                    vec!['a', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
                    vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
                    vec!['a', 'c', 'c', 's', 'z', '{', 'x', 'k'], // E -> {
                    vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
                    vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i']
                ],
                Config {
                    start: (0, 0),
                    end: (5, 2)
                }
            )
        )
    }

    const SMALL_INPUT: &str = "abc
bcd
cde";

    #[test]
    fn test_graph_creation() {
        assert_eq!(
            create_elevation_graph(create_grid_and_config(SMALL_INPUT).0, (0, 0)), // start at (0,0)
            HashMap::from([
                ((1, 0), ((0, 0), 1)),
                ((0, 1), ((0, 0), 1)),
                ((0, 2), ((0, 1), 2)),
                ((1, 1), ((0, 1), 2)),
                ((2, 0), ((1, 0), 2)),
                ((1, 2), ((0, 2), 3)),
                ((2, 1), ((1, 1), 3)),
                ((2, 2), ((1, 2), 4)),
            ])
        )
    }
}
