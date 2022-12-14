mod forest;

use forest::create_view_score_tracker;

use crate::forest::{create_forest, create_height_tracker};

pub fn puzzle1(input: &str) -> usize {
    let forest = create_forest(input);
    let tracker = create_height_tracker(&forest);

    tracker.iter().flatten().filter(|b| **b).count()
}

// multi crate crane
pub fn puzzle2(input: &str) -> usize {
    let forest = create_forest(input);
    let tracker = create_view_score_tracker(&forest);

    *tracker.iter().flatten().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, 8);
    }
}
