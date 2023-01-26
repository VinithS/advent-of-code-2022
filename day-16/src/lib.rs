mod parser;
mod valve;

pub fn puzzle1(_input: &str) -> usize {
    todo!()
}

pub fn puzzle2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    #[ignore]
    fn test_puzzle1() {
        assert_eq!(puzzle1(INPUT), 1651);
    }

    #[test]
    #[ignore]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, 0);
    }
}
