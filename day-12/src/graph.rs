use core::panic;
use std::collections::{HashMap, HashSet};

pub fn create_elevation_graph_bfs(
    grid: Vec<Vec<char>>,
    start_points: HashSet<(usize, usize)>,
) -> HashMap<(usize, usize), ((usize, usize), usize)> {
    let grid_row = grid.len();
    let grid_col = grid[0].len();

    let mut graph: HashMap<(usize, usize), ((usize, usize), usize)> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();

    for starting_point in start_points {
        queue.push(starting_point);
        visited.insert(starting_point);
        graph.insert(starting_point, (starting_point, 0));
    }

    while !queue.is_empty() {
        let c_loc: (usize, usize) = queue.remove(0);

        let current_elevation: char = grid[c_loc.0][c_loc.1];

        for neighbor in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            // cannot simply cast to usize because integer
            // implementation is modulo. Negative get very large as unsigned.
            let n_loc: (usize, usize) = get_normalized_new_loc(&c_loc, neighbor);

            if (n_loc.0 < grid_row)
                && (n_loc.1 < grid_col) // row & col check
                // elevation check
                && is_traversible(current_elevation, grid[n_loc.0][n_loc.1])
                // visited check
                && !visited.contains(&n_loc)
            {
                let len_to_curr = match graph.get(&c_loc) {
                    Some((_, length)) => *length + 1,
                    None => 1,
                };

                graph.insert(n_loc, (c_loc, len_to_curr));
                visited.insert(n_loc);
                queue.push(n_loc);
            }
        }
    }

    graph
}

pub fn create_elevation_graph_dfs(
    grid: Vec<Vec<char>>,
    start_points: HashSet<(usize, usize)>,
) -> HashMap<(usize, usize), usize> {
    let grid_row = grid.len();
    let grid_col = grid[0].len();

    let mut graph: HashMap<(usize, usize), usize> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();

    for starting_point in start_points {
        queue.push(starting_point);
        visited.insert(starting_point);
        graph.insert(starting_point, 0);
    }

    while !queue.is_empty() {
        let c_loc: (usize, usize) = queue.remove(0);

        let current_elevation: char = grid[c_loc.0][c_loc.1];

        for neighbor in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            // cannot simply cast to usize because integer
            // implementation is modulo. Negative get very large as unsigned.
            let n_loc: (usize, usize) = get_normalized_new_loc(&c_loc, neighbor);

            if (n_loc.0 < grid_row)
                && (n_loc.1 < grid_col) // row & col check
                // elevation check
                && is_traversible(current_elevation, grid[n_loc.0][n_loc.1])
                // visited check
                && !visited.contains(&n_loc)
            {
                let len_to_curr = match graph.get(&c_loc) {
                    Some(l) => *l + 1,
                    None => panic!("shouldn't happen?"),
                };

                graph.insert(n_loc, len_to_curr);
                visited.insert(n_loc);
                queue.push(n_loc);
            }
        }
    }

    graph
}

pub fn create_grid_and_config(input: &str, dfs: bool) -> (Vec<Vec<char>>, Config) {
    let mut ep = (0, 0);
    let mut sp: HashSet<(usize, usize)> = HashSet::new();
    sp.insert((0, 0));

    let mut g = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    input.lines().enumerate().for_each(|(r, rval)| {
        rval.chars().enumerate().for_each(|(c, cval)| match cval {
            'E' => {
                ep = (r, c);
                g[r][c] = 123 as char;
            }
            'S' => {
                sp.insert((r, c));
                g[r][c] = 'a';
            }
            'a' => {
                if dfs {
                    sp.insert((r, c));
                }
            }
            _ => {}
        })
    });

    return (g, Config { start: sp, end: ep });
}

fn is_traversible(curr: char, neighbor: char) -> bool {
    if neighbor as i32 - curr as i32 <= 1 {
        true
    } else {
        false
    }
}

// there must be a way to do this through std library?
fn get_normalized_new_loc(c_loc: &(usize, usize), neighbor: &(i32, i32)) -> (usize, usize) {
    let mut rr = c_loc.0 as i32 + neighbor.0;
    let mut rc = c_loc.1 as i32 + neighbor.1;

    if rc.is_negative() {
        rc = 0;
    }
    if rr.is_negative() {
        rr = 0;
    }

    (rr as usize, rc as usize)
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub start: HashSet<(usize, usize)>,
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
            create_grid_and_config(INPUT, false), // bfs
            (
                vec![
                    vec!['a', 'a', 'b', 'q', 'p', 'o', 'n', 'm'], // S -> a
                    vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
                    vec!['a', 'c', 'c', 's', 'z', '{', 'x', 'k'], // E -> '{' (2 = row, 5 = col)
                    vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
                    vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i']
                ],
                Config {
                    start: HashSet::from([(0, 0)]),
                    end: (2, 5)
                }
            )
        );

        assert_eq!(
            create_grid_and_config(INPUT, true), // dfs
            (
                vec![
                    vec!['a', 'a', 'b', 'q', 'p', 'o', 'n', 'm'], // S -> a
                    vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
                    vec!['a', 'c', 'c', 's', 'z', '{', 'x', 'k'], // E -> '{' (2 = row, 5 = col)
                    vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
                    vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i']
                ],
                Config {
                    start: HashSet::from([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (0, 1)]),
                    end: (2, 5)
                }
            )
        );
    }

    const SMALL_INPUT: &str = "abc
bcd
cde";

    #[test]
    fn test_bfs_graph_creation() {
        let grid = create_grid_and_config(SMALL_INPUT, false);
        let config = grid.1;

        assert_eq!(
            create_elevation_graph_bfs(grid.0, config.start), // start at (0,0)
            HashMap::from([
                ((0, 0), ((0, 0), 0)),
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

    #[test]
    fn test_dfs_graph_creation() {
        let grid = create_grid_and_config(SMALL_INPUT, true);
        let config = grid.1;

        assert_eq!(
            create_elevation_graph_dfs(grid.0, config.start), // start at (0,0)
            HashMap::from([
                ((0, 0), 0),
                ((0, 1), 1),
                ((0, 2), 2),
                ((1, 0), 1),
                ((1, 1), 2),
                ((1, 2), 3),
                ((2, 0), 2),
                ((2, 1), 3),
                ((2, 2), 4),
            ])
        )
    }
}
