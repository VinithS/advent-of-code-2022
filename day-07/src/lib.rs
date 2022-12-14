mod dir;
mod parser;

use std::{collections::HashMap, vec};

use dir::{Resources, ROOT_DIR};

use parser::{shell_parser, Cd, Cmd};

use crate::dir::DELIMITER;

// Find all of the directories with a total size of at most 100000.
// What is the sum of the total sizes of those directories?
pub fn puzzle1(input: &str) -> u32 {
    const SIZE_LIMIT: u32 = 100000;

    let all_dirs = get_all_dirs(input);

    return generate_size_tree(&all_dirs)
        .values()
        .filter(|&&s| s <= SIZE_LIMIT)
        .sum();
}

pub fn puzzle2(input: &str) -> u32 {
    const MIN_FREE: i32 = 30000000;
    const TOT_SIZE: i32 = 70000000;

    let all_dirs: HashMap<String, Vec<Resources>> = get_all_dirs(input);
    let size_tree: HashMap<String, u32> = generate_size_tree(&all_dirs);

    let root_dir_size = size_tree.get("").unwrap();

    let space_needed = MIN_FREE - TOT_SIZE - (*root_dir_size as i32); // required - Total - Consumed

    let mut all_sizes: Vec<u32> = size_tree.values().cloned().collect();
    all_sizes.sort_unstable(); // faster but will re-order equal values

    for size in &all_sizes {
        if space_needed - (*size as i32) <= 0 {
            return *size;
        }
    }
    return 0;
}

fn generate_size_tree(all_dirs: &HashMap<String, Vec<Resources>>) -> HashMap<String, u32> {
    // dbg!(&all_dirs);

    let mut size_tree: HashMap<String, u32> = HashMap::new();
    for (path, resources) in all_dirs.iter() {
        let dirs: Vec<&str> = match (*path).as_str() {
            ROOT_DIR => vec![""],
            _ => path.split(DELIMITER).collect(),
        };

        let size = sum_file_sizes(resources);
        // dbg!(&path, &dirs, &size_tree, size);

        // cannot just increment size for all in the path
        // size of dir at level n = n * size
        for idx in 0..dirs.len() {
            let true_path = dirs[0..=idx].join("").to_string();
            // dbg!(&true_path);
            size_tree
                .entry(true_path)
                .and_modify(|val| *val += size)
                .or_insert(size);
        }
        // dbg!(&size_tree);
    }
    size_tree
}

fn get_all_dirs(input: &str) -> HashMap<String, Vec<Resources>> {
    let commands = shell_parser(input).unwrap().1;

    let mut all_dirs: HashMap<String, Vec<Resources>> = HashMap::new();
    let mut curr_stack: Vec<&str> = vec![];

    for cmd in commands.iter() {
        match cmd {
            // I think this implies we can go back to prev dir from root using `cd ..`
            Cmd::Cd(Cd::Root) => curr_stack.push(""), // push nothing, append it in path name generator
            Cmd::Cd(Cd::Down(name)) => curr_stack.push(name),
            Cmd::Cd(Cd::Up) => {
                curr_stack.pop();
            }
            Cmd::Ls(resources) => {
                let dirpath = dir::generate_path(&curr_stack);
                // clone it here
                let mut dirfiles: Vec<Resources> =
                    resources.iter().filter(|r| r.is_file()).copied().collect();
                let map_entry = all_dirs.entry(dirpath);
                map_entry.or_default().append(&mut dirfiles);
            }
        }
    }
    // dbg!(&all_dirs);
    all_dirs
}

fn sum_file_sizes(files: &[Resources]) -> u32 {
    files
        .iter()
        .map(|r| match r {
            Resources::File(_, s) => s,
            _ => &0,
        })
        .sum::<u32>()
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
    fn test_puzzle2() {
        let result = puzzle2(INPUT);
        assert_eq!(result, 24933642);
    }
}
