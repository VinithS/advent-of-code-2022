use std::collections::HashSet;

// full overlap
pub fn puzzle1(input: &str) -> String {
    return run_puzzle(input, check_does_not_contain);
}

// any overlap
pub fn puzzle2(input: &str) -> String {
    return run_puzzle(input, check_contains);
}

fn run_puzzle(input: &str, func: fn(&HashSet<String>, &HashSet<String>) -> bool) -> String {
    let mut counter: i32 = 0;

    for line in input.lines() {
        let sections: Vec<&str> = line.split(",").collect();
        let sec1: HashSet<String> = get_start_end(sections.get(0).unwrap())
            .map(|num| num.to_string())
            .collect();

        let sec2: HashSet<String> = get_start_end(sections.get(1).unwrap())
            .map(|num| num.to_string())
            .collect();

        // dbg!(&sec1, &sec2);
        if func(&sec1, &sec2) || func(&sec2, &sec1) {
            counter += 1;
        }
    }

    return counter.to_string();
}

fn check_contains(s1: &HashSet<String>, s2: &HashSet<String>) -> bool {
    for number in s1 {
        if s2.contains(number) {
            return true;
        }
    }
    return false;
}

fn check_does_not_contain(s1: &HashSet<String>, s2: &HashSet<String>) -> bool {
    for number in s1 {
        if !s2.contains(number) {
            return false;
        }
    }
    return true;
}
// get a range object from [start end]
fn get_start_end(section: &str) -> std::ops::Range<i32> {
    let ints: Vec<i32> = section.split("-").map(|s| s.parse().unwrap()).collect();
    return *ints.get(0).unwrap()..(*ints.get(1).unwrap() + 1); // last num inclusive
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, "4");
    }
}
