mod monkey;
mod parser;

pub fn puzzle1(input: &str) -> String {
    let x = |a: i32, b: i32| a + b;

    for round in 0..20 {}
    return "".to_string();
}

// multi crate crane
pub fn puzzle2(input: &str) -> String {
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1";

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, "");
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, "");
    }
}
