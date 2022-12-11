use std::collections::HashSet;

pub fn puzzle1(input: &str) -> usize {
    return find_first_n_uniq(4, input);
}

pub fn puzzle2(input: &str) -> usize {
    return find_first_n_uniq(14, input);
}

fn find_first_n_uniq(n: usize, string: &str) -> usize {
    for idx in 0..string.len() {
        if idx + n > string.len() - 1 {
            return 0;
        }

        let chars: HashSet<char> = string[idx..idx + n].chars().collect();
        // dbg!(&idx, &chars);
        if chars.len() == n {
            // dbg!("soln", idx + 4);
            return idx + n;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: (&str, usize, usize) = ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19);
    const T2: (&str, usize, usize) = ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23);
    const T3: (&str, usize, usize) = ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23);
    const T4: (&str, usize, usize) = ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29);
    const T5: (&str, usize, usize) = ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26);

    #[test]
    fn test_puzzle1() {
        let r1 = puzzle1(T1.0);
        let r2 = puzzle1(T2.0);
        let r3 = puzzle1(T3.0);
        let r4 = puzzle1(T4.0);
        let r5 = puzzle1(T5.0);
        assert_eq!(r1, T1.1);
        assert_eq!(r2, T2.1);
        assert_eq!(r3, T3.1);
        assert_eq!(r4, T4.1);
        assert_eq!(r5, T5.1);
    }

    #[test]
    fn test_puzzle2() {
        let r1 = puzzle2(T1.0);
        let r2 = puzzle2(T2.0);
        let r3 = puzzle2(T3.0);
        let r4 = puzzle2(T4.0);
        let r5 = puzzle2(T5.0);
        assert_eq!(r1, T1.2);
        assert_eq!(r2, T2.2);
        assert_eq!(r3, T3.2);
        assert_eq!(r4, T4.2);
        assert_eq!(r5, T5.2);
    }
}
