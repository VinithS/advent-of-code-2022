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

// forest has to be square
pub fn create_height_tracker(forest: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let n = forest.len();
    // by default all trees are visible
    let mut tracker = vec![vec![true; n]; n];

    for i in 1..n - 1 {
        for j in 1..n - 1 {
            tracker[i][j] = is_tree_visible((i, j), forest);
        }
    }

    return tracker;
}

pub fn create_view_score_tracker(heights: &Vec<Vec<u32>>) -> Vec<Vec<usize>> {
    let n = heights.len();
    // all trees at the edges will have 0 score.
    let mut tracker = vec![vec![0; n]; n];

    for i in 1..n - 1 {
        for j in 1..n - 1 {
            tracker[i][j] = calc_view_score((i, j), heights);
        }
    }

    return tracker;
}

fn calc_view_score(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> usize {
    let top = top_view_score(loc, tree_heights);
    let bot = bot_view_score(loc, tree_heights);
    let left = left_view_score(loc, tree_heights);
    let right = right_view_score(loc, tree_heights);

    return top * bot * left * right;
}

fn left_view_score(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> usize {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_y = loc.1;

    let mut counter = 0;
    while curr_y > 0 {
        curr_y -= 1;
        if tree_heights[loc.0][curr_y] >= curr_height {
            counter += 1;
            break;
        }
        counter += 1;
    }

    return counter;
}
fn right_view_score(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> usize {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_y = loc.1;

    let mut counter = 0;
    while curr_y < tree_heights.len() - 1 {
        curr_y += 1;
        if tree_heights[loc.0][curr_y] >= curr_height {
            counter += 1;
            break;
        }
        counter += 1;
    }

    return counter;
}
fn top_view_score(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> usize {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_x = loc.0;

    let mut counter = 0;
    while curr_x > 0 {
        curr_x -= 1;
        if tree_heights[curr_x][loc.1] >= curr_height {
            counter += 1;
            break;
        }
        counter += 1;
    }

    return counter;
}
fn bot_view_score(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> usize {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_x = loc.0;

    let mut counter = 0;
    while curr_x < tree_heights.len() - 1 {
        curr_x += 1;
        if tree_heights[curr_x][loc.1] >= curr_height {
            counter += 1;
            break;
        }
        counter += 1;
    }

    return counter; // default visible
}

fn is_tree_visible(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> bool {
    if loc.0 == 0 || loc.1 == 0 {
        return true;
    }

    check_top(loc, tree_heights)
        || check_bot(loc, tree_heights)
        || check_left(loc, tree_heights)
        || check_right(loc, tree_heights)
}

fn check_left(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> bool {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_y = loc.1;

    while curr_y > 0 {
        curr_y -= 1;

        if tree_heights[loc.0][curr_y] >= curr_height {
            return false;
        }
    }

    return true; // default visible
}

fn check_right(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> bool {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_y = loc.1;

    while curr_y < tree_heights.len() - 1 {
        curr_y += 1;

        if tree_heights[loc.0][curr_y] >= curr_height {
            return false;
        }
    }

    return true; // default visible
}

fn check_top(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> bool {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_x = loc.0;

    while curr_x > 0 {
        curr_x -= 1;

        if tree_heights[curr_x][loc.1] >= curr_height {
            return false;
        }
    }

    return true; // default visible
}
fn check_bot(loc: (usize, usize), tree_heights: &Vec<Vec<u32>>) -> bool {
    let curr_height = tree_heights[loc.0][loc.1];
    let mut curr_x = loc.0;

    while curr_x < tree_heights.len() - 1 {
        curr_x += 1;

        if tree_heights[curr_x][loc.1] >= curr_height {
            return false;
        }
    }

    return true; // default visible
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
    fn test_visible_trees() {
        let vf: Vec<Vec<u32>> = create_forest(FOREST);

        assert_eq!(true, is_tree_visible((0, 0), &vf));
        assert_eq!(true, is_tree_visible((1, 0), &vf));

        assert_eq!(false, is_tree_visible((1, 3), &vf));
        assert_eq!(false, is_tree_visible((2, 2), &vf));
        assert_eq!(false, is_tree_visible((3, 1), &vf));
        assert_eq!(false, is_tree_visible((3, 3), &vf));

        assert_eq!(true, is_tree_visible((4, 4), &vf));
    }

    #[test]
    fn test_tracker_creation() {
        let vf = create_forest(FOREST);
        let result = create_height_tracker(&vf);

        assert_eq!(
            [
                [true, true, true, true, true].to_vec(),
                [true, true, true, false, true].to_vec(),
                [true, true, false, true, true].to_vec(),
                [true, false, true, false, true].to_vec(),
                [true, true, true, true, true].to_vec(),
            ]
            .to_vec(),
            result
        )
    }

    #[test]
    fn test_view_score() {
        let vf = create_forest(FOREST);
        let result = create_view_score_tracker(&vf);

        assert_eq!(
            [
                [0, 0, 0, 0, 0].to_vec(),
                [0, 1, 4, 1, 0].to_vec(),
                [0, 6, 1, 2, 0].to_vec(),
                [0, 1, 8, 3, 0].to_vec(),
                [0, 0, 0, 0, 0].to_vec()
            ]
            .to_vec(),
            result
        )
    }
}
