pub fn puzzle1(input: &str) -> String {
    return "4".to_owned();
}

pub fn puzzle2(input: &str) -> String {
    return "4".to_owned();
}

/// --
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, "4");
    }
}
