use std::collections::HashSet;

pub fn puzzle1(input: &str) -> i32 {
    let total_prio_sum: i32 = input
        .lines() // equivalent to split("\n")
        .map(|rucksack: &str| {
            let sack_size = rucksack.len();
            let left: HashSet<char> = rucksack[0..sack_size / 2].chars().collect();
            let right: HashSet<char> = rucksack[sack_size / 2..sack_size].chars().collect();

            println!("{:?} - {:?}", left, right);

            let intersection: HashSet<&char> = left.intersection(&right).collect();

            println!("{:?}", intersection);

            let sack_sum = chars_to_prio_sum(intersection);

            println!("sum: {}", sack_sum);
            return sack_sum;
        })
        .sum();

    return total_prio_sum;
}

pub fn puzzle2(input: &str) -> String {
    return "4".to_owned();
}

fn chars_to_prio_sum(char_set: HashSet<&char>) -> i32 {
    return char_set
        .iter()
        .map(|c: &&char| **c as u32)
        .map(|c| ascii_to_prio(c))
        .fold(0, |acc: i32, e: i32| acc + e);
}

fn ascii_to_prio(char: u32) -> i32 {
    const LC_DELTA: i32 = 96;
    const UC_DELTA: i32 = 38;

    // lowercase = 97..123;
    // uppercase = 65..91;

    if char < 91 {
        return char as i32 - UC_DELTA;
    } else {
        return char as i32 - LC_DELTA;
    }
}

/// --
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, "4");
    }
}
