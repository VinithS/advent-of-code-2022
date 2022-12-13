mod dir;
mod parser;

use parser::shell_parser;

// Find all of the directories with a total size of at most 100000.
// What is the sum of the total sizes of those directories?
pub fn puzzle1(input: &str) -> usize {
    // const SIZE_LIMIT: i32 = 100000;

    dir_helper(input);
    return 0;
}

pub fn puzzle2(_input: &str) -> usize {
    return 0;
}

fn dir_helper(input: &str) {
    let (input, commands) = shell_parser(input).unwrap();

    dbg!(input);
    dbg!(commands);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn test_puzzle1() {
        let result = puzzle1(INPUT);
        assert_eq!(result, 95437);
    }

    #[test]
    #[ignore]
    fn test_puzzle2() {
        let _ = puzzle2(INPUT);
        // assert_eq!(result, "");
    }
}
