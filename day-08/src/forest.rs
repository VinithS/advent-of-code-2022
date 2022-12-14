#[derive(Clone)]
pub struct Stat(pub bool, pub usize);

impl Stat {
    fn merge4(&self, s1: Stat, s2: Stat, s3: Stat) -> Stat {
        Stat(self.0 || s1.0 || s2.0 || s3.0, self.1 * s1.1 * s2.1 * s3.1)
    }
}

pub fn create_forest(input: &str) -> Vec<Vec<u32>> {
    // initialize board
    let n = input.split("\n").next().unwrap().len();
    let mut board: Vec<Vec<u32>> = vec![vec![0; n]; n]; // N x N board

    for (row, trees) in input.split("\n").enumerate() {
        // reverse to start idx at bot left.
        for (col, height) in trees.chars().enumerate() {
            board[row][col] = height.to_digit(10).unwrap();
        }
    }

    return board;
}

pub fn create_stat_tracker(tree_heights: &Vec<Vec<u32>>) -> Vec<Vec<Stat>> {
    let n = tree_heights.len();
    // by default all trees are visible
    let mut tracker = vec![vec![Stat(true, 0); n]; n];

    for i in 1..n - 1 {
        for j in 1..n - 1 {
            tracker[i][j] = calcuate_forest_stats((i, j), tree_heights)
        }
    }

    return tracker;
}

fn calcuate_forest_stats(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> Stat {
    let t = check_stats_top(loc, tree_heights);
    let b = check_stats_bot(loc, tree_heights);
    let l = check_stats_left(loc, tree_heights);
    let r = check_stats_right(loc, tree_heights);

    t.merge4(b, l, r)
}

fn check_stats_top(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> Stat {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_x = loc.0;

    let mut counter = 0;
    while curr_x > 0 {
        curr_x -= 1;
        if tree_heights[curr_x][loc.1] >= curr_height {
            return Stat(false, counter + 1);
        }
        counter += 1;
    }

    return Stat(true, counter);
}

fn check_stats_bot(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> Stat {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_x = loc.0;

    let mut counter = 0;
    while curr_x < tree_heights.len() - 1 {
        curr_x += 1;
        if tree_heights[curr_x][loc.1] >= curr_height {
            return Stat(false, counter + 1);
        }
        counter += 1;
    }

    return Stat(true, counter);
}

fn check_stats_left(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> Stat {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_y = loc.1;

    let mut counter = 0;
    while curr_y > 0 {
        curr_y -= 1;
        if tree_heights[loc.0][curr_y] >= curr_height {
            return Stat(false, counter + 1);
        }
        counter += 1;
    }

    return Stat(true, counter);
}

fn check_stats_right(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> Stat {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_y = loc.1;

    let mut counter = 0;
    while curr_y < tree_heights.len() - 1 {
        curr_y += 1;

        if tree_heights[loc.0][curr_y] >= curr_height {
            return Stat(false, counter + 1);
        }
        counter += 1;
    }

    return Stat(true, counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    const FOREST: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_create_forest() {
        let result = create_forest(FOREST);

        assert_eq!(
            [
                [3, 0, 3, 7, 3].to_vec(),
                [2, 5, 5, 1, 2].to_vec(),
                [6, 5, 3, 3, 2].to_vec(),
                [3, 3, 5, 4, 9].to_vec(),
                [3, 5, 3, 9, 0].to_vec(),
            ]
            .to_vec(),
            result
        );
    }

    #[test]
    fn test_tracker_creation() {
        let vf = create_forest(FOREST);
        let stats = create_stat_tracker(&vf);

        let visibility: Vec<Vec<bool>> = stats
            .iter()
            .map(|v| v.iter().map(|s| s.0).collect())
            .collect();

        let view_score: Vec<Vec<usize>> = stats
            .iter()
            .map(|v| v.iter().map(|s| s.1).collect())
            .collect();

        assert_eq!(
            [
                [(true), (true), (true), (true), (true)].to_vec(),
                [(true), (true), (true), false, (true)].to_vec(),
                [(true), (true), false, (true), (true)].to_vec(),
                [(true), false, (true), false, (true)].to_vec(),
                [(true), (true), (true), (true), (true)].to_vec(),
            ]
            .to_vec(),
            visibility
        );

        assert_eq!(
            [
                [0, 0, 0, 0, 0].to_vec(),
                [0, 1, 4, 1, 0].to_vec(),
                [0, 6, 1, 2, 0].to_vec(),
                [0, 1, 8, 3, 0].to_vec(),
                [0, 0, 0, 0, 0].to_vec()
            ]
            .to_vec(),
            view_score
        )
    }
}
